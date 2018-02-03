use futures::{Async, Poll, Stream};
use super::test::Test;
use super::test_fixture::TestFixture;
use super::test_result::{TestResult, TestResultMethods};

pub struct ParallelTestScheduler<F>
where
    F: TestFixture,
{
    fixture: F,
    test_queue: Vec<F::TestSpec>,
    test_executions: Vec<F::Test>,
}

impl<F> ParallelTestScheduler<F>
where
    F: TestFixture,
{
    pub fn new(fixture: F) -> Self {
        Self {
            fixture,
            test_queue: Vec::new(),
            test_executions: Vec::new(),
        }
    }

    pub fn add<S>(&mut self, test_spec: S)
    where
        S: Into<F::TestSpec>,
    {
        self.test_queue.push(test_spec.into());
    }

    pub fn add_all<S, I>(&mut self, test_specs: I)
    where
        S: Into<F::TestSpec>,
        I: IntoIterator<Item = S>,
    {
        let test_specs = test_specs.into_iter();
        let (minimum_specs, _maximum_specs) = test_specs.size_hint();

        self.test_queue.reserve(minimum_specs);

        for test_spec in test_specs {
            self.test_queue.push(test_spec.into());
        }
    }

    fn start_queued_tests(&mut self) {
        for test_spec in self.test_queue.drain(0..) {
            self.test_executions.push(self.fixture.start(test_spec))
        }
    }

    fn next_test_result(&mut self) -> Poll<Option<<Self as Stream>::Item>, ()> {
        let next_ready_result = self.test_executions
            .iter_mut()
            .zip(0..)
            .filter_map(|(execution, index)| match execution.poll() {
                Ok(Async::NotReady) => None,
                poll_result => Some((poll_result, index)),
            })
            .next();

        if let Some((poll_result, index)) = next_ready_result {
            self.test_executions.remove(index);

            Ok(Async::Ready(Some(TestResult::from_poll(poll_result))))
        } else {
            if self.all_tests_finished() {
                Ok(Async::Ready(None))
            } else {
                Ok(Async::NotReady)
            }
        }
    }

    fn all_tests_finished(&self) -> bool {
        self.test_queue.is_empty() && self.test_executions.is_empty()
    }
}

impl<F> Stream for ParallelTestScheduler<F>
where
    F: TestFixture,
{
    type Item = TestResult<<F::Test as Test>::Error>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.start_queued_tests();
        self.next_test_result()
    }
}
