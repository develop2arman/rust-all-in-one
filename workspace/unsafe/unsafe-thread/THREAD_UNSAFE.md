> Another use case for unsafe is implementing an unsafe trait. 
> A trait is unsafe when at least one of its methods has some invariant that By using unsafe impl, we’re promising that > we’ll uphold the invariants that the compiler can’t verify.
> We can declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the 

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
