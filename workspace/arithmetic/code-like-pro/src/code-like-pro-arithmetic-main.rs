#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-arithmetic_bin --bin code-like-pro-arithmetic-main```
///
/// ```cargo doc  --package code-like-pro-arithmetic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-pro-arithmetic_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
fn main() {
    // println!("{}", 1 / 0); does not compile

    let one = 1;
    let zero = 0;
    // println!("{}", one / zero); does not compile

    let one = 1;
    let zero = one - 1;
    // println!("{}", one / zero); still doesn't compile

    let one = { || 1 }();
    let zero = { || 0 }();
    println!("{}", one / zero); // panics here
}
