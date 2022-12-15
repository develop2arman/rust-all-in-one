#![allow(dead_code, unused_variables)]

/// rust-doc-dangling-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-dangle_bin --bin  rust-doc-dangling-bin-ex-1```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
/// 
/// # Arguments
///
/// * `Arg1` - This is the integer number to verb the struc-name
///
/// # Return
/// `nothing`
///
/// ## Example
/// ```rust,no_run,compile_fail,ignore
///fn main() {
///let reference_to_nothing = dangle();
///}
///
///fn dangle() -> &String {
///    let s = String::from("hello");
///
///    &s
///}
/// ```

fn main() {

 println!("Printed:{:?}","Hi");


}
