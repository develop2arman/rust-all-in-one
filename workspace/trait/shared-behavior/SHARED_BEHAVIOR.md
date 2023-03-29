[[TRAIT_BOUND]]

[[NONE_OBJECT_SAFE]]

[[SB_TRAIT_BUILTIN]]

[[SB_AGGRIGATOR]]

---

## Generic traits
> Two such examples are is the **From<T> and Into<T> traits**, which allow from conversion from a type to a type T and vice versa.
> generic traits can get quite **verbose** when they are declared with three or four generic types.

```rust,compile_fail,no_run,ignore
pub trait From<T> {
    fn from(T) -> Self;
}
```

## Trait bounds
> In the implementation of outline_print, we want to use the Display trait’s functionality. Therefore, we need to specify that the OutlinePrint trait will work only for types that also implement Display and provide the functionality that OutlinePrint needs. We can do that in the trait definition by specifying OutlinePrint: Display. 

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

> Notice the : Loadable part. This is how we specify a trait bound. Trait bounds allow us to constrain the range of parameters that a generic API can accept. Specifying a trait bound on a generic item is similar to how we specify types for variables, but here the variable is the generic type T and the type is some trait, such as**T: SomeTrait**. 
> Trait bounds are almost **always needed when defining generic functions**. If one defines a generic function that takes T without any trait bounds, we cannot call any of the methods since Rust does not know what implementation to use for the given methods.
>> The **where** clause decouples the trait bound from the function signature and makes it readable.
> 
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

## Example Trait Bound

In the implementation of outline_print, we want to use the Display trait’s functionality. Therefore, we need to specify that the OutlinePrint trait will work only for types that also implement Display and provide the functionality that OutlinePrint needs. We can do that in the trait definition by specifying OutlinePrint: Display. 
This technique is similar to adding a trait bound to the trait.
we can use the to_string function that is automatically implemented for any type that implements Display.

```rust
use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) { 
         let output = self.to_string();
         output
    }
}
```

## Aggregator
> Other crates that depend on the aggregator crate can also bring the Summary trait into scope to implement Summary on their own types. One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
> > For example, **we can implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate functionality**, because the type Tweet is local to our aggregator crate. We can also implement Summary on Vec<T> in our aggregator crate, because the trait Summary is local to our aggregator crate.
> > But **we can’t implement external traits on external types**. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate. 
> This restriction is part of a property called **coherence**, and more specifically the **orphan rule**, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. 
> Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

## Orphan Rule-Coherence
The idea of trait coherence is that there **should be exactly one implementation of a trait on a type that implements it**. This should be quite obvious since, with two implementations there would be ambiguity in what to choose between the two.

Another rule that might confuse many about traits is the orphan rule. The orphan rule, in simple words, **states that we cannot implement external traits on external types.**
To word it in another way, *either the trait must be defined by you* if you are implementing something on an external type, or your type should be defined by you when you are implementing an external trait. This rules out the possibility of having conflicts in overlapping trait implementations across crates.

> But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are defined in the standard library and aren’t local to our aggregator crate.

> This restriction is part of a property of programs called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. 

> Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

## Glossery

> `fully qualified syntax` : <Type as Trait>::function(receiver_if_method, next_arg, ...);

> `bound`  : 'where Self: Sized'


---

> `tags` [[orphan_rule]] [[coherence]]
