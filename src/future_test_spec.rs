use std::fmt::Display;

use futures::{Future, Poll};

use super::test::Test;

pub struct FutureTestSpec<E> {
    name: String,
    future: Box<Future<Item = (), Error = E>>,
}

impl<E> FutureTestSpec<E> {
    pub fn new(
        name: String,
        future: Box<Future<Item = (), Error = E>>,
    ) -> Self {
        FutureTestSpec {
            name,
            future,
        }
    }
}

impl<E> Test for FutureTestSpec<E>
where
    E: Display,
{
    type Error = E;

    fn name(&self) -> &str {
        &self.name
    }

    fn poll_test(&mut self) -> Poll<(), Self::Error> {
        self.future.poll()
    }
}
