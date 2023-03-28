
[[mr_closure]]

[[rd_closure]]

[[pnkfx_closure]]

[[er_closure]]

---

## Closure Vs Function
what sets them apart from functions is that they are also **aware of the environment** they are declared within and can reference any variable from their environment. The way they reference variables from their environment is determined by how the variable is used inside the closure.

## Closure

> Closure to code blocks that are **encapsulated** in Rust.

> Closures are also known as anonymous functions and lambda functions. 

> (|...|) followed by curly brackets ({...}). The pair of vertical bars lets you define arguments. Lambda functions in 

> Rust can read variables from within their scope. These are closures.

> Unlike regular functions, lambda functions cannot be defined in global scope(out of main).

A closure, by default, will try to **capture** the variable in the most flexible way possible. Only when the programmer needs a certain way of capturing the value will they coerce to the programmer's intent. That won't make much sense unless we see different kinds of closures in action. *Closures under the hood are anonymous structs that implement three traits(Fn, FnOnce, FnMut)* that represent how closures access their environment. We will look at the three traits (ordered from least restrictive to most restrictive) next.

### Fn
Closures that **access variables only for read access** implement the Fn trait. Any value they access are as reference types (&T). This is the default mode of borrowing the closures assumes.

### FnOnce
Closures that take ownership of the data they read from their environment get implemented with FnOnce. The name signifies that this closure can only be called once and, because of that, the variables are available only once. This is the least recommended way to construct and use closures, because **you cannot use the referenced variables later**

### FnMut
When the compiler figures out that a closure **mutates a value referenced from the environment**, the closure implements the FnMut trait.

## Returning Closures
you’re not allowed to use the function pointer fn as a return type, for example.
The following code tries to return a closure directly, but it won’t compile:
```rust,compile_fail,no_run
fn returns_closure() -> dyn Fn(i32) -> i32 {//error doesn't have a size known at compile-time
    |x| x + 1
}
```

The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure. We saw a solution to this problem earlier. We can use a trait object:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```


## Glossery

  > `closure`:	<>  , || {}   , Closures are represented by traits, so they cannot be a return type, let consume_and_return_x = move || x;

> `tags` 
