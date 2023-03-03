> Here is an example trait that is not object safe:

```rust,compile_fail,no_run
trait SomeTrait {
    fn foo(&self) -> int { ... }
    
    // Object-safe methods may not return `Self`:
    fn new() -> Self;
}
```

> Splitting a trait:

>One option is to split a trait into object-safe and non-object-safe parts. We hope that this will lead to better design. We are not sure how much code this will affect, it would be good to have data about this.

```rust,compile_fail,no_run
trait SomeTrait {
    fn foo(&self) -> int { ... }
}

trait SomeTraitCtor : SomeTrait {
    fn new() -> Self;
}
```

> Adding a where-clause:
> Sometimes adding a second trait feels like overkill. In that case, it is often an option to simply add a where Self:Sized clause to the methods of the trait that would otherwise violate the object safety rule.

```rust,compile_fail,no_run
trait SomeTrait {
    fn foo(&self) -> int { ... }
    
    fn new() -> Self
        where Self : Sized; // this condition is new
}
```
