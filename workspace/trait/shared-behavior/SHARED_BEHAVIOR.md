

## Generic traits
> Two such examples are is the **From<T> and Into<T> traits**, which allow from conversion from a type to a type T and vice versa.
> generic traits can get quite **verbose** when they are declared with three or four generic types.

```rust,compile_fail,no_run,ignore
pub trait From<T> {
    fn from(T) -> Self;
}
```

## Trait bounds

```rust,compile_fail,no_run,ignore
fn load<T: Loadable>(&self, entity: T) { .. }
```

> Notice the : Loadable part. This is how we specify a trait bound. Trait bounds allow us to constrain the range of parameters that a generic API can accept. Specifying a trait bound on a generic item is similar to how we specify types for variables, but here the variable is the generic type T and the type is some trait, such as**T: SomeTrait**. 
> Trait bounds are almost **always needed when defining generic functions**. If one defines a generic function that takes T without any trait bounds, we cannot call any of the methods since Rust does not know what implementation to use for the given methods.
>> The **where** clause decouples the trait bound from the function signature and makes it readable.
> 
> We can specify **trait bounds on types** too:

```rust,compile_fail
struct Foo<T: Display> {..}
```
```rust,compile_fail,no_run,ignore
struct TEST<T: Display> {..}
```
```rust
fn main(){
    println!("Hi amran")
}
```
```rust
    println!("Hi amran")
```

```rust
# fn main(){
    println!("Hi amran")
# }
```

## Aggregator
> Other crates that depend on the aggregator crate can also bring the Summary trait into scope to implement Summary on their own types. One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. For example, we can implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate functionality, because the type Tweet is local to our aggregator crate. We can also implement Summary on Vec<T> in our aggregator crate, because the trait Summary is local to our aggregator crate.
> But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate. This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
