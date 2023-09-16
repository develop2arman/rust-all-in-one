#![allow(dead_code, unused_variables)]
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;


/// rust-doc-smartpointer-arc-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-arc_bin --bin rust-doc-smartpointer-arc-ex-5```
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
/// `unimplemented`
///
/// ## Example
/// `TODO`


fn main(){
let val = Arc::new(AtomicUsize::new(5));

for _ in 0..10 {
    let val = Arc::clone(&val);

    thread::spawn(move || {
        let v = val.fetch_add(1, Ordering::SeqCst);
        println!("{v:?}");
    });
}
}
