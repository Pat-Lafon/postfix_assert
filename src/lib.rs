#![deny(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

/*!
This crate provides trait implementations for standard library structs with properties that are commonly asserted. These assertions are different than other crates in that they abandon the common `assert!(x.is_none());` statement style in favor of an inline, readable `x.assert_none();`. Importantly, these `assert_*` methods return the reference that they are called on so that you can interweave your assertions with your code instead of them needing to be on their separate lines.

# Example

```
fn computation_producing_some() -> Option<u32> {
    Some(1)
}

fn computation_producing_none_from_some(x: Option<u32>) -> Option<u32> {
    None
}
computation_producing_some()
    .assert_some()
    .and(computation_producing_none_from_some)
    .assert_none()
```

# Crate Features

* **debug** -
  When enabled, this will cause change all assertions to `debug_assertions` which only run in debug mode but not in release mode. The overhead for assertions is generally small, especially if the compiler can optimize them away. When they are costly, this feature only enables asserts when you are trying to debug your code. When this code runs in release mode, all methods in this library are optimized away.

# Other Assertion Crates

I am starting to compile a list of crates that empower assertions in some form. I use this for my benefit in two ways: I learn about new crates, which I may want to use in my code and, I can ~steal~ borrow ideas for useful assertions from these other crates. This list will always be incomplete and may or may not have my comments alongside each crate.

* <https://crates.io/crates/assert>
* <https://crates.io/crates/spectral>
* <https://crates.io/crates/totems>
* <https://crates.io/crates/proc_static_assertions>
* <https://crates.io/crates/galvanic-assert>
* <https://crates.io/crates/chek>
* <https://crates.io/crates/all_asserts>
* <https://crates.io/crates/assert-panic>
* <https://crates.io/crates/assert2>
* <https://crates.io/crates/assert2>
* <https://crates.io/crates/const_fn_assert>
* <https://crates.io/crates/more-asserts>
* <https://crates.io/crates/power-assert>
* <https://crates.io/crates/assert_fs>
* <https://crates.io/crates/assert_matches>
* <https://crates.io/crates/claim>
* <https://crates.io/crates/assert-impl>
* <https://crates.io/crates/assert_cli>
* <https://crates.io/crates/rustspec_assertions>
* <https://crates.io/crates/similar-asserts>
* <https://crates.io/crates/sixarm_assert>
* <https://crates.io/crates/assert_ng>
* <https://crates.io/crates/assert_bound>
* <https://crates.io/crates/assert_ne>
* <https://crates.io/crates/static_assert>
* <https://crates.io/crates/assert_into>
* <https://crates.io/crates/assert-next>
* <https://crates.io/crates/assert_panic_free>
* <https://crates.io/crates/enum-unitary>
*/

mod assert;
mod option;
pub use option::*;
