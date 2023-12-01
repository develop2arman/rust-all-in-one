
[[edu-macro]]

[[rd-macro]]

---

Macros are the Rust metaprogramming feature that creates code in **compile-time**. Through this, we can **save duplication**, but we need to be careful because we could make a mess.

> The type of parameter or the fragment specifier is expr, but Rust offers other parameters as well:

- [*] item: An item.
- [*] block: A block expression.
- [*] stmt: A statement.
- [*] pat: A pattern.
- [*] expr: An expression.
- [*] ty: A type.
- [*] ident: An identifier or a keyword.
- [*] path: A type path.
- [*] tt: A token.
- [*] meta: The contents of an attribute.
- [*] lifetime: A lifetime token.
- [*] vis: A possibly empty visibility qualifier.
- [*] literal: A literal expression.


---

> The best-case scenario for a macro is to avoid duplication.
```rust

macro_rules! create_model {
    ($name:ident) => {
        #[allow(unused_variables)]
        #[allow(dead_code)]
        #[derive(Debug)]
        struct $name {
            product_details: Vec<ProductDetail>,
            date: Date,
            subtotal: Option<f64>,
            total_tax: f64,
            total: Option<f64>
        }
    }
}
create_model!(Sale);
create_model!(Purchase);
create_model!(Budget);
```
> `tags` 
