use std::marker::PhantomData;

use super::test::Test;
use super::test_fixture::TestFixture;

pub struct EmptyTestFixture<S, T>
where
    S: Into<T>,
    T: Test,
{
    _test_spec_type: PhantomData<S>,
    _test_type: PhantomData<T>,
}

impl<S, T> EmptyTestFixture<S, T>
where
    S: Into<T>,
    T: Test,
{
    pub fn new() -> Self {
        EmptyTestFixture {
            _test_spec_type: PhantomData,
            _test_type: PhantomData,
        }
    }
}

impl<S, T> TestFixture for EmptyTestFixture<S, T>
where
    S: Into<T>,
    T: Test,
{
    type TestSpec = S;
    type Test = T;

    fn start(&mut self, test_spec: Self::TestSpec) -> Self::Test {
        test_spec.into()
    }
}
