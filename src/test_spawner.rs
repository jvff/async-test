use super::into_test::IntoTest;

pub trait TestSpawner {
    type TestSetup: IntoTest;

    fn spawn(&mut self) -> Self::TestSetup;
}
