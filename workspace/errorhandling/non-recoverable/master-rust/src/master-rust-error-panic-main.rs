#![allow(dead_code, unused_variables)]
use std::thread;

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p master-rust-error-panic_bin --bin master-rust-error-panic-main```
///
/// ```cargo doc  --package master-rust-error-panic_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-error-panic_bin ```
///
/// ## What
/// `panic_unwinding`
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
///


fn alice() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        bob();
    })
}

fn bob() {
    malice();
}

fn malice() {
    panic!("malice is panicking!");
}

fn main() {
    let child = alice();
    let _ = child.join();

    bob();
        println!("This is unreachable code");
}
