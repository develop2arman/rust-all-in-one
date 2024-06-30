
How to capture ref in match for getting selected value by match?

```rust,no_run,compile_fail
    match bag.food {
        Food::Cake => println!("I got cake"),
        ref a => println!("I got {:?}", a)
    }
```

---

> `tags`  [[ref]]
