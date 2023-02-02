[[CLOSURE]]

[[FUNCTION]]

[[LAZY]]

---

## Jargon

> Functional programming jargon: “to cons x onto y” informally means to construct a new container instance by putting the element x at the start of this new container, followed by the container y.

> Vec<T> is a better choice to use. Other, more complex recursive data types are useful in various situations, but by starting with the cons list, we can explore how boxes let us define a recursive data type without much distraction.

```rust,no_run_compile_fail
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {}
```

## Func Vs Closure

> Unlike closures, fn is a type rather than a trait

> Functions can implement all three of the Fn traits too. 
 - If what we want to do doesn’t require capturing a value from the environment,
 - we can use a function rather than a closure where we need something that implements an Fn trait.
 - in the case of closure it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as memoization or lazy evaluation.but there is a little a bit issue about closure that is good using memory for caching and avoid call fun but:

> - When a closure captures a value from its environment,
- it uses memory to store the values for use in the closure body. 
- This use of memory is overhead

> - All closures implement FnOnce because they can all be called at least once. 
- Closures that don’t move the captured variables also implement FnMut,
- and closures that don’t need mutable access to the captured variables also implement Fn

---
