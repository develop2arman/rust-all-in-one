#![allow(dead_code, unused_variables)]
///
/// brain-teaser-educative-ex-11
///
/// # Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p brain-teaser-educative_bin --bin brain-teaser-educative-ex-11```
///
/// ```cargo doc  --package brain-teaser-educative_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package brain-teaser-educative_bin```
///
/// ```cargo clippy --package brain-teaser-educative_bin --bin brain-teaser-educative-ex-11```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// ## Solution
///
/// //```rust,no_run,compile_fail```
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,no_run,compile_fail /// ```
///
fn main() {
    let c = Box::new([0u32; 10_000_000]);
    println!("{}", c.len());
}
