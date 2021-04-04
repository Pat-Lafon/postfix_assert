#[cfg(not(feature = "debug"))]
#[macro_export]
#[doc(hidden)]
macro_rules! my_assert {
    ($($tt:tt)*) => {
        assert!($($tt)*)
    }
}

#[cfg(feature = "debug")]
#[macro_export]
#[doc(hidden)]
macro_rules! my_assert {
    ($($tt:tt)*) => {
        debug_assert!($($tt)*)
    }
}
