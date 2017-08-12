use std::mem;

use futures::{Async, Future, IntoFuture, Poll};
use super::test_result::TestResult;
use super::test_spawner::TestSpawner;

pub struct TestScheduler<S, T, E>
where
    S: TestSpawner<Test = T>,
    T: IntoFuture<Item = TestResult<E>, Error = ()>,
{
    spawner: S,
    tests: Vec<Box<FnMut(&mut T)>>,
    test_executions: Vec<T::Future>,
    test_results: Vec<TestResult<E>>,
}

impl<S, T, E> TestScheduler<S, T, E>
where
    S: TestSpawner<Test = T>,
    T: IntoFuture<Item = TestResult<E>, Error = ()>,
{
    pub fn new(spawner: S) -> Self {
        Self {
            spawner,
            tests: Vec::new(),
            test_executions: Vec::new(),
            test_results: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, test_setup: F)
    where
        F: FnMut(&mut T) + 'static,
    {
        self.tests.push(Box::new(test_setup));
    }
}

impl<S, T, E> Future for TestScheduler<S, T, E>
where
    S: TestSpawner<Test = T>,
    T: IntoFuture<Item = TestResult<E>, Error = ()>,
{
    type Item = Vec<TestResult<E>>;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        for mut test_setup_function in self.tests.drain(0..) {
            let mut test = self.spawner.spawn();

            test_setup_function(&mut test);

            self.test_executions.push(test.into_future());
        }

        let test_executions_count = self.test_executions.len();
        let poll_results = self.test_executions
            .iter_mut()
            .map(|execution| execution.poll())
            .zip(0..test_executions_count)
            .rev()
            .collect::<Vec<_>>();

        for (poll_result, index) in poll_results {
            match poll_result {
                Ok(Async::Ready(result)) => {
                    self.test_results.push(result);
                    self.test_executions.remove(index);
                }
                Ok(Async::NotReady) => {}
                Err(_) => panic!("Fatal test execution failure"),
            }
        }

        if self.tests.is_empty() && self.test_executions.is_empty() {
            let test_results =
                mem::replace(&mut self.test_results, Vec::new());

            Ok(Async::Ready(test_results))
        } else {
            Ok(Async::NotReady)
        }
    }
}
