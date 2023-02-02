#![allow(dead_code, unused_variables)]
/// master-rust-function-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-function_bin --bin master-rust-function-ex-2```
///
/// ```cargo doc  --package master-rust-function_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-function_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// `The following code demonstrates how we can do this entirely at runtime.using the include_bytes! macro, which also reads the file at compile time.Without the const function, all this would be done at runtime.`
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
const fn read_header(a: &[u8]) -> (u8, u8, u8, u8) {
    (a[0], a[1], a[2], a[3])
}

const FILE_HEADER: (u8,u8,u8,u8) = read_header(include_bytes!("./const_fn_file.rs"));

fn main() {
    println!("{:?}", FILE_HEADER);
}
