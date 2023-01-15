#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p rust-doc-shared-behavior-nondispatchable_bin --bin rust-doc-shared-behavior-nondispatchable-main```
///
/// ```cargo doc  --package rust-doc-shared-behavior-nondispatchable_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-shared-behavior-nondispatchable_bin ```
///
/// ## What
/// `nondispatchable shared behaviour tarits`
///
/// ## How
/// > tweet.summarize() is a shared behavior
/// > Other crates that depend on the nondispatchable crate can also bring the Summary trait into scope to implement Summary on their own types. One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. For example, we can implement standard library traits like Display on a custom type like Tweet as part of our nondispatchable crate functionality, because the type Tweet is local to our nondispatchable crate. We can also implement Summary on Vec<T> in our nondispatchable crate, because the trait Summary is local to our nondispatchable crate.
/// > But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our nondispatchable crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our nondispatchable crate. This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `This code prints 1 new tweet: horse_ebooks: of course, as you probably already know, people.`
///  > output: `1 new tweet: (Read more from @horse_ebooks...)`
/// ## Example
///  `TODO`
///
///

fn main() {
// This trait is object-safe, but these methods cannot be dispatched on a trait object.
trait NonDispatchable {
    // Non-methods cannot be dispatched.
    fn foo() where Self: Sized {}
    // Self type isn't known until runtime.
    fn returns(&self) -> Self where Self: Sized;
    // `other` may be a different concrete type of the receiver.
    fn param(&self, other: Self) where Self: Sized {}
    // Generics are not compatible with vtables.
    fn typed<T>(&self, x: T) where Self: Sized {}
}

struct S;
impl NonDispatchable for S {
    fn returns(&self) -> Self where Self: Sized { S }
}
let obj: Box<dyn NonDispatchable> = Box::new(S);
// obj.returns(); // ERROR: cannot call with Self return
// obj.param(S);  // ERROR: cannot call with Self parameter
// obj.typed(1);  // ERROR: cannot call with generic type
}
