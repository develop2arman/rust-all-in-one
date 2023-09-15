
> `tags` [[match_binding]] [[rd-match]] #scrutinee

---

## Match Bindings
[nicer-match-bindings](https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#nicer-match-bindings)

> `self` has type `&List`, and `*self` has type `List`, matching on a concrete type `T` is preferred over a match on a reference `&T` after Rust 2018 you can use self here and tail (with no ref) below as well,rust will infer &s and ref tail. 
> the compiler automatically references the Some, and since we're borrowing, name is bound as ref name automatically as well. If we were mutating:

```rust
fn hello(arg: &mut Option<String>) {
    match arg {
        Some(name) => name.push_str(", world"),
        None => (),
    }
}
```
> The compiler will automatically borrow by mutable reference, and name will be bound as ref mut too.

## Concise_control

```rust,no_run,compile_fail
Coin::Quarter(state) => println!("State quarter from {:?}!", state),
if let Some(max) = config_max {}
```

### A match with an Arm
But what use is a type you can never create values for? Recall the code from Listing 2-5, part of the number guessing game; we’ve reproduced a bit of it here in Listing 19-26.

> A match with an arm that ends in continue:

```rust
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,            
        };
```        


At the time, we skipped over some details in this code. In Chapter 6 in “The match Control Flow Operator” section, we discussed that match arms must all return the same type. So, for example, the following code doesn’t work:

> This code does not compile!

```rust,no_run,compile_fail
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "hello",
    };
```
As you might have guessed, continue has a ! value. That is, when Rust computes the type of guess, it looks at both match arms, the former with a value of u32 and the latter with a ! value. Because ! can never have a value, Rust decides that the type of guess is u32.

The formal way of describing this behavior is that expressions of type ! can be coerced into any other type. We’re allowed to end this match arm with continue because continue doesn’t return a value; instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.

## A match with Panic

```rust,no_run
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```
Rust sees that val has the type T and **panic! has the type !**, so the result of the overall match expression is T. This code works because **panic! doesn’t produce a value**; it ends the program. In the None case, we won’t be returning a value from unwrap, **so this code is valid**. [[panic]]

One final expression that has the **type ! is a loop**:
```rust,no_run,compile_fail
    print!("forever ");

    loop {
        print!("and ever ");
    }
```
Here, the loop never ends, so ! is the value of the expression. However, this wouldn’t be true if we included a break, because the *loop would terminate when it got to the break*.

