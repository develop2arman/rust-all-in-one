
[[ria-types-rhs]

[[rd-types-rhs]

[[ria-types-rhs]

---


## Right Hand Side

> or default type parameters, for example the **fragment** <T: std::ops::Add<Output = T>> says that T must implement trait std::ops::Add. Using a single type variable T with the trait bound ensures that arguments i and j, as well as the result type, are the same type and that their type supports addition.

```rust,no_run
trait Add<Rhs=Self> {  type Output; fn add(self, rhs: Rhs) -> Self::Output;}
```

---

> `tags` [[rhs]] [[DEFAULT_CONCRETE_TYPE]]
