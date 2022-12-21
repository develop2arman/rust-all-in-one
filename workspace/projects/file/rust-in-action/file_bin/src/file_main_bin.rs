#![allow(dead_code, unused_variables)]
use file_lib::*;

/// Main
///
/// ## Commands
///
///
/// ```cargo run -q -p file_bin --bin file_main_bin```
///
/// ```cargo doc  --package file_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package file_bin```
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
/// `nothing`
///
/// // ```compile_fail,ignore
///


 fn main() {
  let mut f5 = File::new("file-project.txt");

  let mut buffer: Vec<u8> = vec![];

  if f5.read(&mut buffer).is_err() {
    println!("Error checking is working");
  }

  f5 = open(f5).unwrap();
  let f5_length = f5.read(&mut buffer).unwrap();
  f5 = close(f5).unwrap();

  let text = String::from_utf8_lossy(&buffer);


  let f5_name = f5.name();
  let f5_len = f5.len();

  println!("{:?}", f5);
  println!("{} is {} bytes long", f5_name, f5_len);


   println!("{:?}", f5);
   println!("{}", f5);
   println!("{} is {} bytes long", &f5.name, f5_length);
   println!("{}", text);
 }
