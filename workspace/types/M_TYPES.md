[[WRAPPER]]

[[RHS]]

[[COMPUND]]

[[GENERIC]]

[[RETURN]]

[[STATIC]]

[[M_SCAL]]

[[VAR]]

---

## Glossery

> `Local type`: A struct, enum, or union which was defined in the current crate. This is not affected by applied type arguments. struct Foo is considered local, but Vec<Foo> is not. LocalType<ForeignType> is local. Type aliases do not affect locality.

[Type-Layout](https://doc.rust-lang.org/nightly/reference/type-layout.html#type-layout)

> `Nominal types`: Types that can be referred to by a path directly. Specifically enums, structs, unions, and trait objects.
