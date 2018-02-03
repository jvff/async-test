use super::test::Test;

pub trait IntoTest {
    type Test: Test;

    fn into_test(self) -> Self::Test;
}
