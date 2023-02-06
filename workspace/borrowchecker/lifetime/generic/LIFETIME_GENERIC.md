

## Lifetime IO

> The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.
> Lifetimes on function or method parameters are called **input lifetimes**, and lifetimes on return values are called output lifetimes.

```rust,no_run,compile_fail
let result = longest(string1.as_str(), string2);
fun longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
```

---

> Using two lifetime parameters (a and b) indicates that the lifetimes of i and j are decoupled.
```rust,no_run,compile_fail
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {}
```

## Lifetime Subtyping

> the 'c part, like a type, also guards what is allowed into r.

```rust,no_run,compile_fail
let r: &'c S = &c;
```

---

> This is read as the lifetime 'a outlives 'b or in other words 'b should never live longer than 'a.

```rust,no_run,compile_fail
impl<'a, 'b, S, R> Decoder<'a, 'b, S, R>
where 'a: 'b {}
```

## Lifetime Bounds

> Along with the **Send** bound, which says that this thread **can be sent to threads,** we also say that the type must live as long as the 'static lifetime.

```rust,no_run,compile_fail
struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(_t: T) where T: Send + 'static {
    // configure the logger here
}
```

---

> `tags` [[Send]] [[LIFETIME_STATIC]]
