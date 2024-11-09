#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-factorial_bin --bin other-factorial-cache```
///
/// ## What
/// `Factorial Cache Profficent Performance`
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
///```rust

use std::collections::HashMap;

struct Factorial {
    cache: HashMap<u64, u64>,
}

impl Factorial {
    fn new() -> Self {
        Factorial {
            cache: HashMap::new(),
        }
    }

    fn factorial(&mut self, n: u64) -> u64 {
        if n == 0 {
            return 1; // Base case: 0! = 1
        }

        // Check if the result is already in the cache
        if let Some(&result) = self.cache.get(&n) {
            return result; // Return the cached value
        }

        // Compute the factorial recursively
        let result = n * self.factorial(n - 1);

        // Store the result in the cache
        self.cache.insert(n, result);

        result
    }
}

fn main() {
    let mut factorial = Factorial::new();

    let num = 10;
    let result = factorial.factorial(num);
    println!("Factorial of {} is {}", num, result);
}