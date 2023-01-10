#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p master-rust-trait-bound-operator_bin --bin master-rust-trait-bound-operator-main```
///
/// ```cargo doc  --package master-rust-trait-bound-operator_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-trait-bound-operator_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// > two kinds of methods within a trait:
/// > self is just a type alias to Self,which refers to the type on which the trait is being implemented
/// > Sample Associated method: pause, Instance methods: play
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
///
use std::ops::Add;

fn add_thing<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

fn main() {
    add_thing(2, 2);
}
