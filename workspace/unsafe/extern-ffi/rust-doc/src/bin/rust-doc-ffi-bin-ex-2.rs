#![allow(dead_code, unused_variables)]
#[macro_use] extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

/// rust-doc-ffi-main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-ffi_bin --bin rust-doc-ffi-bin-ex-2```
///
/// ## What
/// `TODO`
///
/// ## How
/// `This usage of extern does not require unsafe.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore

///The enum_primitive crate also gives us some helpful boilerplate functions for converting a u8 to an Option<Animal>

enum_from_primitive! {
#[derive(Debug, PartialEq)]
enum FooBar {
    Foo = 17,
    Bar = 42,
    Baz,
}
}

fn main() {
    assert_eq!(FooBar::from_i32(17), Some(FooBar::Foo));
    assert_eq!(FooBar::from_i32(42), Some(FooBar::Bar));
    assert_eq!(FooBar::from_i32(43), Some(FooBar::Baz));
    assert_eq!(FooBar::from_i32(91), None);
}
