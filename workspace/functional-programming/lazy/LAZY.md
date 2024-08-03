
[[mr_lazy]]

[[rd_lazy]]

---

## Dynamic Static

For example, you can't create a HashMap as a static value because it requires a heap allocation. Fortunately, we can have HashMap and other dynamic collection types such as Vec as global statics too, using a third-party crate called lazy_static. This crate exposes the lazy_static! macro, which can be used to initialize any dynamic type that's accessible globally from anywhere in the program. Here's a snippet of how to initialize a Vec that can be mutated from multiple threads(Items declared within the lazy_static! macro are required to implement the Sync trait):

```rust
use std::sync::Mutex;
lazy_static! {
    static ref ITEMS: Mutex<Vec<u64>> = {
        let mut v = vec![];
        v.push(9);
        v.push(2);
        v.push(1);
        Mutex::new(v)
    }
}
```
> Items declared within the lazy_static! macro are required to implement the Sync trait. This means if we want a mutable static, we have to use a multithreaded type such as Mutex or RwLock instead of  RefCell.

## Memorization-Lazy-Evaluation
>We can create a struct that will hold the closure and the resulting value of calling the closure.The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as memoization or lazy evaluation.

```rust,no_run
let fn=|num| -> {}
```

### Convert to memoization or lazy evaluation
> Fn, FnMut, or FnOnce. We’ll discuss the difference between these traits in the “Capturing the Environment with Closures”

 - FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined.
 > The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
 - FnMut can change the environment because it mutably borrows values.
 - Fn borrows values from the environment immutably.
 
 - FnOnce: takes the whole value
 - FnMut: takes a mutable reference
 - Fn: takes a regular reference 
 
```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}


```


---

> `tags` [[memorization]] [[cache]] [[memory]] [[FnOnce]] [[FnMut]] [[Fn]] [[evaluation]] [[macro]] [[static]]
