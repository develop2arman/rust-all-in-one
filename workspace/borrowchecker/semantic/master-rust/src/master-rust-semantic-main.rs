#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-semantic_bin --bin master-rust-semantic-main```
///
/// ```cargo doc  --package master-rust-semantic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-semantic_bin```
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
/// `Owned string!`
///
/// ## Example
/// `TODO`
///```
fn main() {
    let mut a = String::from("Owned string");
    let a_ref = &mut a;
    a_ref.push('!');
    println!("{}", a);
}
