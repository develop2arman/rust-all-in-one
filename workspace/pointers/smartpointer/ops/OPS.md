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

## How Deref Coercion Interacts with Mutability
Rust does deref coercion when it finds types and trait implementations in three cases:

* From &T to &U when T: Deref<Target=U>
* From &mut T to &mut U when T: DerefMut<Target=U>
* From &mut T to &U when T: Deref<Target=U>

The first two cases are the same as each other except that the second implements mutability. The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently. The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that. Therefore, **Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.**

---

> `tags` [[Deref]] [[ops]]  [[Box]] #deref_coercion_feature
