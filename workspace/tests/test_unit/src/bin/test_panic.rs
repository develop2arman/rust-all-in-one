#![allow(dead_code, unused_variables)]

/// Bin
///
/// ## Commands
///
/// ```cargo test -q -p test_unit_bin --bin test_panic```
///
/// ```cargo doc  --package test_unit_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package test_unit_bin```
///
/// ## What
/// `Unit Test`
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
/// `TODO`

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}