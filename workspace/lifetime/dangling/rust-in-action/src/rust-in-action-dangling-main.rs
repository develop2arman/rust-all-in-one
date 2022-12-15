#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-dangling_bin --bin rust-in-action-dangling-main```
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
/// ```compile_fail,ignore
/// fn main() {
/// let mut grains: Vec<Cereal> = vec![];
/// grains.push(Cereal::Rye);
/// drop(grains);
/// println!("{:?}", grains); //^^^^^^ value borrowed here after move
/// ```

 fn main() {
     let mut grains: Vec<Cereal> = vec![];
     grains.push(Cereal::Rye);
     drop(grains);
 }
 #[derive(Debug)]
 enum Cereal {
     Barley, Millet, Rice,
     Rye, Spelt, Wheat,
 }
