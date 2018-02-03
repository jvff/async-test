use super::test::Test;

pub trait TestFixture {
    type TestSpec;
    type Test: Test;

    fn start(&mut self, spec: Self::TestSpec) -> Self::Test;
}
