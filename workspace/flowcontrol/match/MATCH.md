[[rd-match]]

---

## Match Bindings
[nicer-match-bindings](https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#nicer-match-bindings)

> `self` has type `&List`, and `*self` has type `List`, matching on a concrete type `T` is preferred over a match on a reference `&T` after Rust 2018 you can use self here and tail (with no ref) below as well,rust will infer &s and ref tail. 
> the compiler automatically references the Some, and since we're borrowing, name is bound as ref name automatically as well. If we were mutating:

```rust
fn hello(arg: &mut Option<String>) {
    match arg {
        Some(name) => name.push_str(", world"),
        None => (),
    }
}
```
> The compiler will automatically borrow by mutable reference, and name will be bound as ref mut too.

## Concise_control

```rust,no_run,compile_fail
Coin::Quarter(state) => println!("State quarter from {:?}!", state),
if let Some(max) = config_max {}
```

---

> `tags` [[match_binding]]
