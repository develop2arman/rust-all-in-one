#![allow(dead_code, unused_variables)]

/// ex-1
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-enum_bin --bin packtpub-enum-ex-1```
///
/// ```cargo doc  --package packtpub-enum_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-enum_bin```
///
/// ## What
// `refutable_pattern`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your enum/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore


enum Container {
    Item(u64),
    //Empty
}

fn main() {
    let mut item = Container::Item(56);
    let Container::Item(it) = item;
}
