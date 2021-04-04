0.1.1 (2021-04-04)
==================

This update contains a small change to the README documentation and implements the `PostfixAssertOption` trait for `&Option<T>`. Rename Github repository to match crate name.

0.1.0 (2021-04-03)
==================

The first release! In response to the removal of `unwrap_none` in <https://github.com/rust-lang/rust/pull/83349>, I've created this crate both for my personal use and to get experience testing/documenting a crate. This adds the `assert_none` and `assert_some` methods to `Option<T>` via the `PostfixAssertOption` trait.
