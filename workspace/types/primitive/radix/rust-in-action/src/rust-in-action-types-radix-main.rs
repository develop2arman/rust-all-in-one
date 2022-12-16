#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-radix_bin --bin rust-in-action-types-radix-main```
///
/// ```cargo doc  --package rust-in-action-types-radix_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-radix_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [verb] the [your struct/func name]
///
/// # Return
/// `23`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
  let three = 0b11; // <1>
  let thirty = 0o36; // <2>
  let three_hundred = 0x12C; // <3>

  println!("base 10:  {}    {}       {}", three, thirty, three_hundred);
  println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
  println!("base 8:   {:o}  {:o}       {:o}", three, thirty, three_hundred);
  println!("base 16:  {:x}  {:x}       {:x}", three, thirty, three_hundred);
}
