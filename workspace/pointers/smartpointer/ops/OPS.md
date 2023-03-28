[[er-smartpointer-ops]]

[[rd-smartpointer-ops]]

---

## Treating Smart Pointers Like Regular References with the Deref Trait

> By implementing Deref in such a way that a smart pointer can be treated like a regular reference.

> Note: there’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: our version will not store its data on the heap. We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.



```rust,compile_fail.no_run
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); //error[E0277]: can't compare `{integer}` with `&{integer}`
}
```
> Comparing a number and a reference to a number isn’t allowed because they’re different types. We must use the dereference operator to follow the reference to the value it’s pointing to:

> we set y to be an instance of **a Box<T> pointing to a copied value of x rather than a reference pointing to the value of x**.
```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
> `Attention to example folder/a customize box with Deref feature`'

> This code without deref coercions is harder to read, write, and understand with all of these symbols involved. Deref coercion allows Rust to handle these conversions for us automatically.

> When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type. The number of times that Deref::deref needs to be inserted *is resolved at compile time*, so there is **no runtime penalty for taking advantage of deref coercion!**

---

> `tags` [[Deref]] [[ops]]  [[Box]] #deref_coercion_feature
