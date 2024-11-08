#![allow(dead_code, unused_variables)]


/// rust-doc-cpu-main-ex-1
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-cpu_bin --bin rust-doc-cpu-main-ex-1```
///
/// ```cargo doc  --package rust-doc-cpu_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-cpu_bin ```
///
/// ## What
/// `Multi Impl`
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
/// //``rust,no_run,compile_fail,ignore
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
