use std::time::Duration;

pub trait PostfixAssertDuration {
    fn assert_zero(&self) -> ();
}

impl PostfixAssertDuration for Duration {
    fn assert_zero(&self) -> () {
        assert!(self.is_zero())
    }
}
