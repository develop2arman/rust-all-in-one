[[rd_function]]

[[re_function]]

---

## Const function
This means that they **cannot take mutable** arguments to any type. They also **cannot** include operations that are dynamic such as a **heap** allocation.they are evaluated at **compile time**.

> The following code demonstrates how we can do this entirely at runtime.using the include_bytes! macro, which also reads the file at compile time.Without the const function, all this would be done at runtime:

```rust,no_run,compile_fail
const fn read_header(a: &[u8]) -> (u8, u8, u8, u8) {
    (a[0], a[1], a[2], a[3])
}

const FILE_HEADER: (u8,u8,u8,u8) = read_header(include_bytes!("./const_fn_file.rs"));

fn main() {
    println!("{:?}", FILE_HEADER);
}
```

## Function Pointers

Unlike closures, **fn is a type** rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.

Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure. *It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.*

That said, one example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but *C doesn’t have closures*.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

As an example of where you could use either a closure defined inline or a named function, let’s look at a use of the map method provided by the Iterator trait in the standard library. To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:

```rust
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
```
> Or we could name a function as the argument to map instead of the closure, like this:
```rust
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
```

```rust
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

```

## Glossery

  > `closure`:	<>  , || {}   , Closures are represented by traits, so they cannot be a return type, let consume_and_return_x = move || x;

---

> `tags` [[file]] [[compile_time]] [[runtime]] [[const]] [[macro]]
