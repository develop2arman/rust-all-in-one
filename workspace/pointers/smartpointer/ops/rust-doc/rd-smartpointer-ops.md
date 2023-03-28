
>Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *. The deref coercion feature also lets us write more code that can work for either references or smart pointers.

> If Rust didn’t implement deref coercion, we would have to write the code in: `hello(&(*m)[..]);`

```rust,no_run,compile_fail
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);// =  hello(&m);
}
```

---

> `tags` [[Deref]] [[DerefMut]]
