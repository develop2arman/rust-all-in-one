#![allow(dead_code, unused_variables)]
use pretty_assertions::{assert_eq, assert_ne};

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p test_pretty_bin --bin test_pretty-main```
///
/// ```cargo doc  --package test_pretty_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package test_pretty_bin```
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
#[cfg(test)]
mod tests {

use super::*;
use pretty_assertions::{assert_eq, assert_ne};


    #[test]
    fn diff(){
        let x= "Hello world";
        let y = "Helo World";
        
        assert_eq!(x, y);
    }


#[derive(PartialEq, Eq, Hash)]
struct Student{
    first_name: String,
    last_name: String,    
}

#[derive(PartialEq, Eq, Hash)]
struct Grade{
        code: u32,
    }
}


fn main() {
    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
    let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});

    assert_eq!(x, y);
    println!("{}", "Arman");
}

