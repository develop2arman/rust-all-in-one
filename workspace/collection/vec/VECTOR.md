[[ria-vec]]

---

> A "Vec" type is shorthand for vector. Vectors are arrays that will dynamically expand when needed.
> The underscore asks the the compiler to infer the type of the vector's elements.

```rust,no_run,compile_fail
let fields: Vec<_> = record 
```


> Make this a while let statement - remember that vector.pop also adds another layer of Option<T>
> You can stack `Option<T>`'s into while let and if let
  
```rust,no_run,compile_fail
while let Some(Some(value)) = optional_values_vec.pop() {
      println!("current value: {}", value);
}
```
