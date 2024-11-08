use std::collections::HashMap;

fn factorial(n: u128) -> Option<u128> {
    let mut memo = HashMap::<u128, u128>::new();

    fn recursive_factorial(n: u128, memo: &mut HashMap<u128, u128>) -> Option<u128> {
        match n {
            0 | 1 => Some(1),
            _ => {
                // Check if the value is already computed
                if let Some(&result) = memo.get(&n) {
                    return Some(result);
                }
                
                // Compute factorial recursively
                let result = recursive_factorial(n - 1, memo)?.checked_mul(n)?; // Use checked multiplication to avoid overflow
                memo.insert(n, result); // Store result in cache
                Some(result)
            }
        }
    }

    recursive_factorial(n, &mut memo)
}

fn main() {
    println!("Factorial of 20: {:?}", factorial(20));
    println!("Factorial of 21: {:?}", factorial(21));
}