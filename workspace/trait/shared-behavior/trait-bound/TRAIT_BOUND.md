[[operate]]

[[edu-trait-bound]]

[[ms-trait-bound]]

---


## Trait bounds
> In the implementation of outline_print, we want to use the Display trait’s functionality. Therefore, we need to specify that the OutlinePrint trait will work only for types that also **implement Display and provide the functionality that OutlinePrint needs.** We can do that in the trait definition by specifying OutlinePrint: Display. 

> This technique is similar to adding a trait bound to the trait.
>we can use the to_string function that is automatically implemented for any type that implements Display.

```rust
use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {  let output = self.to_string();}
}
```
>

```rust,compile_fail,no_run,ignore
fn load<T: Loadable>(&self, entity: T) { .. }
```

> Notice the : Loadable part. This is how we specify a trait bound. *Trait bounds allow us to constrain the range of parameters that a generic API can accept*. *Specifying a trait bound on a generic item is similar to how we specify types for variables,* but here the variable is the generic type T and the type is some trait, such as **T: SomeTrait**. 

> Trait bounds are almost **always needed when defining generic functions**. 

> *If one defines a generic function that takes T without any trait bounds, we cannot call any of the methods since Rust does not know what implementation to use for the given methods.*

> The **where** clause decouples the trait bound from the function signature and makes it readable.

> We can specify **trait bounds on types** too:

```rust,compile_fail,no_run,ignore
struct Foo<T: Display> {..}
```
### Trait bounds with impl trait syntax

Instead of specifying T: Display, we directly use impl Display. This is the impl trait syntax. This provides advantages in cases where we want to **return a complex or unrepresentable type, such as a closure** from a function. Without this syntax, you had to return it by putting it behind a pointer using the Box smart pointer type, which involves heap allocation.

```rust
use std::fmt::Display;

fn show_me(val: impl Display) {
    println!("{}", val);
}

fn main() {
    show_me("Trait bounds are awesome");
}
```

>The impl trait syntax for trait bounds is mostly recommended to be used as return types from functions. Using it in parameter position means that we can't use the turbofish operator. it should only be used when we don't have a concrete type available to us, as is the case with closures.

```rust
use std::fmt::Display;

fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{{}}}", val)
}

fn main() {
    println!("{}", surround_with_braces("Hello"));
}
```


## Why we need trait bound?

> educative-rust-trait-ex-7.rs


