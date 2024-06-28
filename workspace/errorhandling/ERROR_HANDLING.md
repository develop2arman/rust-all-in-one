
# Recoverable

[[ria-errorhandling]]


[[clp-errorhandling]]


[[edu-errorhandling]]


[[rip-errorhandling]]


[[rd-errorhandling]]


[[mr-errorhandling]]


# Non-Recoverable

[[mr-non-recoverable-errorhandling]]

---

## Prelude

> Most error handling, in general, falls into three categories:

- **Recoverable** errors that are expected to happen due to the **user and the environment** interacting with the program, for example,**a file not found error or a number parse error. Option and Result** for recoverable errors
- **Non-recoverable** errors that violate the contracts or invariants of the program, for example, **index out of bounds or divide by zero non-recoverable errors, there's a mechanism called panic**.
- **Fatal errors** that abort the program immediately. Such situations include **running out of memory, and stack overflow.**

> Rust does not have the notion of #null values, which is infamously quoted as being the #billion-dollar mistake by #Tony_Hoare, who introduced null references in the ALGOL W language back in 1965.

> As an **enum**, they get the ability to store a success state and an error state, while generics allow them to specialize **at compile time** so that they store any value in either state. These types also come with a lot of convenient methods (commonly known as **combinators**) implemented on them, allowing you to consume, compose, or transform the inner values easily.

> encouraged to read and become familiar with their type signature by referring to their documentation:

- **map_err:** This method acts only on Result types and allows **transforming the failed value from E to some other type**, H, but only if the value is an Err value. **map_err is not defined for Option types**, as doing anything with None would be pointless.

- **and_then:** In the case of a failed value, this returns the value as is, but in the case of a successful value, this takes in a closure as the second argument, which acts on the wrapped value and returns the wrapped type. This is useful when **you need to perform transformations on the inner values,** one after another.
  
- **unwrap_or:** This method **extracts the inner success value**, or returns a default one if it's a failed value. You provide the default value to it as a second argument.
  
- **unwrap_or_else:** This method acts the same as the preceding method but computes a different value when it is a failed value by taking a closure as the second argument. In the standard library documentation. Many more of these methods can clean up huge nested match expressions when you’re dealing with errors.
  
- **as_ref:** This method **converts the inner value to a reference** and returns the wrapped value, that is, an Option<&T> or a Result<&T, &E>.
  
- **or/ or_else:** These methods return the value as is if it's a success value, or returns an alternative Ok/Some value, which is provided as the second argument. or_else accepts a closure within which you need to **return a success value**.
  
- **as_mut:** This method **converts the inner value into a mutable reference** and returns the wrapped value, that is, an Option<&mut T> or a Result<&mut T, &mut E>.
>> There are many more that are unique to the Option and Result types.

---

- **ok_or**: This method **converts an Option value to a Result value**, by taking in an error value as a second parameter. A similar variant to this is the ok_or_else method, which should be preferred over this, as it computes the value lazily by taking in a closure.
  
- **ok**: This method converts a Result into an Option consuming self, and **discards the Err value**.


## Operator ?
There is a important notice based on this examples about ?
- `master-rust-error-handling-ex-4`
- `master-rust-error-handling-ex-5`

> > The ? operator abstracts this pattern, making it possible to write the **bytes_to_str** method in a more concise way

> we want to make an early return and propagate the error to the caller.

> The error message also mentioned that ? can be used with Option<T> values as well. As with using ? on Result, you can only use ? on Option in a function that returns an Option. The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early from the function at that point. If the value is Some, the value inside the Some is the resulting value of the expression and the function continues.
> This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

> Sample : propagating errors

```rust
match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
}
```
> what the ? operator does: error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as there’s an impl From<OtherError> for ReturnedError to define the conversion in the trait’s from function, the ? operator takes care of calling the from function automatically.

> In the context of Listing 9-7, the ? at the end of the File::open call will return the value inside an Ok to the variable f. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.

> The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the ?

> Either option&result :
> you can’t mix and match. The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases, there are methods like the ok method on Result or the ok_or method on Option that will do the conversion explicitly.


## Warning Handling


> We have two ways of **handling this warning:**
- Handle both the Ok and Err cases as before for the Result value returned by the read_to_string method
- Assign the return value to a special variable _ (underscore), which lets the compiler know that we want to ignore the value

For cases where we don't care about the value, we can use the second approach and so the read_to_string line changes as follows:

```rust
let _ = file.read_to_string(&mut s);
```

With that change, the code compiles without warnings. However, you should handle the return value and try not to use the catch all underscore variable.

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

## Difference between println! vs eprintln!

Macro std::eprintln

```rust
macro_rules! eprintln {
    () => { ... };
    ($fmt:expr) => { ... };
    ($fmt:expr, $($arg:tt)*) => { ... };
}
```

Macro for printing to the standard error, with a newline.
Equivalent to the println! macro, except that output goes to **io::stderr instead of io::stdout**. See println! for example usage.
Use eprintln! only for error and progress messages. Use println! instead for the primary output of your program.
Panics

`Panics if writing to io::stderr fails.`

Examples:

```rust
eprintln!("Error: Could not complete task");
```

---

`rustc --explain E0599`
