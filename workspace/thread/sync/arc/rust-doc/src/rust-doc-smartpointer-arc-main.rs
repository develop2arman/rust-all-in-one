#![allow(dead_code, unused_variables)]
use std::sync::Arc;
use std::thread;
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-arc_bin --bin rust-doc-smartpointer-arc-main```
///
/// ```cargo doc  --package rust-doc-smartpointer-arc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-arc_bin```
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


let five = Arc::new(5);

    for _ in 0..10 {
        let five = Arc::clone(&five);

        thread::spawn(move || {
            println!("{five:?}");
        });
    }
}
