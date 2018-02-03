use std::marker::PhantomData;

use super::test::Test;
use super::test_fixture::TestFixture;

pub struct EmptyTestFixture<T>
where
    T: Test,
{
    _test_type: PhantomData<T>,
}

impl<T> EmptyTestFixture<T>
where
    T: Test,
{
    pub fn new() -> Self {
        EmptyTestFixture {
            _test_type: PhantomData,
        }
    }
}

impl<T> TestFixture for EmptyTestFixture<T>
where
    T: Test,
{
    type TestSpec = T;
    type Test = T;

    fn start(&mut self, test: Self::TestSpec) -> Self::Test {
        test
    }
}
