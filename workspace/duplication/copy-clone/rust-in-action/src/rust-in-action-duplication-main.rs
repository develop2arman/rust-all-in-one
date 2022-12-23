#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p rust-in-action-duplication_bin --bin rust-in-action-duplication-main```
///
/// ```cargo doc  --package rust-in-action-duplication_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-duplication_bin ```
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
///
#[derive(Debug,Clone,Copy)]   // <1>
struct CubeSat {
  id: u64,
}

#[derive(Debug,Clone,Copy)]   // <1>
enum StatusMessage {
  Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
  StatusMessage::Ok
}

fn main () {
  let sat_a = CubeSat { id: 0 };

  let a_status = check_status(sat_a.clone());   // <2>
  println!("a: {:?}", a_status.clone());        // <2>

  let a_status = check_status(sat_a);           // <3>
  println!("a: {:?}", a_status);                // <3>
}
