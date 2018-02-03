use std::collections::VecDeque;

use futures::{Async, Poll, Stream};
use super::test::Test;
use super::test_fixture::TestFixture;
use super::test_result::{TestResult, TestResultMethods};

pub struct SequentialTestScheduler<F>
where
    F: TestFixture,
{
    fixture: F,
    test_queue: VecDeque<F::TestSpec>,
    test_execution: Option<F::Test>,
}

impl<F> SequentialTestScheduler<F>
where
    F: TestFixture,
{
    pub fn new(fixture: F) -> Self {
        Self {
            fixture,
            test_queue: VecDeque::new(),
            test_execution: None,
        }
    }

    pub fn add<S>(&mut self, test_spec: S)
    where
        S: Into<F::TestSpec>,
    {
        self.test_queue.push_back(test_spec.into());
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
            self.test_queue.push_back(test_spec.into());
        }
    }

    fn start_next_test(&mut self) {
        if let Some(test_spec) = self.test_queue.pop_front() {
            self.test_execution = Some(self.fixture.start(test_spec));
        }
    }

    fn all_tests_finished(&self) -> bool {
        self.test_execution.is_none() && self.test_queue.is_empty()
    }
}

impl<F> Stream for SequentialTestScheduler<F>
where
    F: TestFixture,
{
    type Item = TestResult<<F::Test as Test>::Error>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        while !self.all_tests_finished() {
            if let Some(mut test_execution) = self.test_execution.take() {
                match test_execution.poll() {
                    Ok(Async::NotReady) => {
                        self.test_execution = Some(test_execution);

                        return Ok(Async::NotReady);
                    }
                    poll_result => {
                        let test_result = TestResult::from_poll(poll_result);

                        return Ok(Async::Ready(Some(test_result)));
                    }
                }
            }

            self.start_next_test();
        }

        Ok(Async::Ready(None))
    }
}
