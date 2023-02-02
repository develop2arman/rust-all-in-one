
## Const function
This means that they **cannot take mutable** arguments to any type. They also **cannot** include operations that are dynamic such as a **heap** allocation.they are evaluated at **compile time**.

> The following code demonstrates how we can do this entirely at runtime.using the include_bytes! macro, which also reads the file at compile time.Without the const function, all this would be done at runtime:

```rust,no_run,compile_fail
const fn read_header(a: &[u8]) -> (u8, u8, u8, u8) {
    (a[0], a[1], a[2], a[3])
}

const FILE_HEADER: (u8,u8,u8,u8) = read_header(include_bytes!("./const_fn_file.rs"));

fn main() {
    println!("{:?}", FILE_HEADER);
}
```

## Glossery

  > `closure`:	<>  , || {}   , Closures are represented by traits, so they cannot be a return type, let consume_and_return_x = move || x;

---

> `tags` [[file]] [[compile_time]] [[runtime]] [[const]] [[macro]]
