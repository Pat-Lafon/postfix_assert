/*!
Postfix assertions for `std::option::Option`.
*/

use std::option::Option;
use crate::my_assert;

/// Describes the `assert_*` methods for `std::option::Option<T>`
///
/// This trait is sealed to allow for additional assertion methods in the future. As such, this trait cannot be implemented by the user.
#[allow(clippy::module_name_repetitions)]
pub trait PostfixAssertOption: private::Sealed {
    /// This checks the Option given and panics if it is not a `Some(T)`. Otherwise it just returns the option back to you.
    fn assert_some(self) -> Self;
    /// This checks the Option given and panics if it is not `None`. Otherwise it just returns the option back to you.
    fn assert_none(self) -> Self;
}

mod private {
    // https://rust-lang.github.io/api-guidelines/future-proofing.html
    pub trait Sealed {}
    impl<T> Sealed for Option<T> {}
    impl<T> Sealed for &Option<T> {}
}

impl<T> PostfixAssertOption for Option<T> {
    #[inline]
    fn assert_some(self) -> Self {
        my_assert!(self.is_some());
        self
    }
    #[inline]
    fn assert_none(self) -> Self {
        my_assert!(self.is_none());
        self
    }
}

impl<T> PostfixAssertOption for &Option<T> {
    #[inline]
    fn assert_some(self) -> Self {
        my_assert!(self.is_some());
        self
    }
    #[inline]
    fn assert_none(self) -> Self {
        my_assert!(self.is_none());
        self
    }
}

#[cfg(all(test, not(feature = "debug")))]
mod tests {
    use super::PostfixAssertOption;

    #[test]
    fn assert_none() {
        let none: Option<u32> = None;
        none.assert_none();
        None::<u32>.assert_none();
    }

    #[test]
    #[should_panic]
    fn assert_none_panic() {
        let none: Option<u32> = Some(1);
        none.assert_none();
    }

    #[test]
    fn assert_some() {
        let none: Option<u32> = Some(1);
        none.assert_some();
        Some(1).assert_some();
    }

    #[test]
    #[should_panic]
    fn assert_some_panic() {
        let none: Option<u32> = None;
        none.assert_some();
    }

    #[test]
    fn assert_option_chaining() {
        let none: Option<u32> = None;
        none.assert_none().or(Some(1)).assert_some();
    }

    #[test]
    #[should_panic]
    fn assert_option_chaining_panic() {
        let none: Option<u32> = None;
        none.assert_none().or(Some(1)).assert_none();
    }
}

#[cfg(all(test, not(debug_assertions), feature = "debug"))]
mod debug_release {
    use super::PostfixAssertOption;

    #[test]
    fn assert_none() {
        let none: Option<u32> = None;
        none.assert_none();
        None::<u32>.assert_none();
    }

    #[test]
    fn assert_none_panic() {
        let none: Option<u32> = Some(1);
        none.assert_none();
    }

    #[test]
    fn assert_some() {
        let none: Option<u32> = Some(1);
        none.assert_some();
        Some(1).assert_some();
    }

    #[test]
    fn assert_some_panic() {
        let none: Option<u32> = None;
        none.assert_some();
    }
}

#[cfg(all(test, debug_assertions, feature = "debug"))]
mod debug {
    use super::PostfixAssertOption;

    #[test]
    fn assert_none() {
        let none: Option<u32> = None;
        none.assert_none();
        None::<u32>.assert_none();
    }

    #[test]
    #[should_panic]
    fn assert_none_panic() {
        let none: Option<u32> = Some(1);
        none.assert_none();
    }

    #[test]
    fn assert_some() {
        let none: Option<u32> = Some(1);
        none.assert_some();
        Some(1).assert_some();
    }

    #[test]
    #[should_panic]
    fn assert_some_panic() {
        let none: Option<u32> = None;
        none.assert_some();
    }
}
