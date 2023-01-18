

[[ria-errorhandling]]


[[clp-errorhandling]]


[[edu-errorhandling]]


---

## Anyhow Library

> The problem with the ? operator is that all errors have to be the same type to work. However, in most cases, we have to deal with different error types. To solve that, we can use the crate anyhow.


```rust,no_run,compile_fail
use anyhow::{anyhow, Result};
use serde_json::Value;

// This is the same function we've seen in the lesson, but we are 
// serializing a json object, taking the value and sum it with the argument.
// Without anyhow this code would have a lot of more lines of code.
fn sum_numbers(number: &str) -> Result<i32> {
    let num = number.parse::<i32>()?;
    let num_json: Value = serde_json::from_str("{\"one\": 12}")?;
    let get_number = num_json.get("one").ok_or(anyhow!("Error getting number"))?;
    let num2: i32 = serde_json::from_value(get_number.clone())?;
    Ok(num + num2)
}

fn main() {
    println!("sum two numbers: {:#?}", sum_numbers("567"));
}
```

Now, things start to get a little bit complicated, but we can follow it. In the above code, we parse a number from a &str type value. This would produce a FromStr::Error type in case of error, which is very different from the error we can obtain from line 6.

In line 7, the method get will return an Option type. We need to convert it to a Result with ok_or. This way, we can use the anyhow crate.

Finally, in line 8, we get a different error.
