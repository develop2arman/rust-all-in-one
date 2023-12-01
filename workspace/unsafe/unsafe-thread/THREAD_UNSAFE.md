
## Implementing an Unsafe Trait

> Another use case for unsafe is implementing an unsafe trait. 

> A trait is unsafe when at least one of its methods has some invariant that By using unsafe impl, we’re promising that > we’ll uphold the invariants that the compiler can’t verify.

> We can declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the 

> By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.

`As an example,` recall the Sync and Send marker traits, the compiler implements these traits automatically if our types are composed entirely of Send and Sync types. 

> If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and **we want to mark that type as Send or Sync, we must use unsafe**. Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or **accessed from multiple threads**; therefore, we need to do those checks manually and indicate as such with unsafe.

```rust,compile_fail,no_run
trait as unsafe too
{
Sync and Send are examples of unsafe traits.
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```
