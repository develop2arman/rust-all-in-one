  #![allow(dead_code, unused_variables, unused_imports)]
 use num::{Float};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-convert-from_bin --bin rust-in-action-convert-from-main```
///
/// ```cargo doc  --package rust-in-action-convert-from_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-convert-from_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`

//  struct Q1_7(i8);  //tuple struct holding a i8 value

//  impl From<T: Float> for Q1_7 {
//     fn from (n:T) -> Self {
//         let val = if n > 1.0 {  //out of bounds numbers are coerced to the maximum of the range
//             1
//         } else if n < -1.0 {
//             -1
//         } else {
//             n * (2**7)
//         }

//         Q1_7(val as i8)
//     }
// }

// impl From for U: Float {
//     fn from(q: Q1_7) -> U {
//         q.0 * (2 ** -7)
//     }
// }

fn main(){
    unimplemented!()
}
