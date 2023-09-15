
[[ria-types-rhs]

[[rd-types-rhs]

[[ria-types-rhs]

---


## Right Hand Side

> or default type parameters, for example the **fragment** <T: std::ops::Add<Output = T>> says that T must implement trait std::ops::Add. Using a single type variable T with the trait bound ensures that arguments i and j, as well as the result type, are the same type and that their type supports addition.

```rust,no_run
trait Add<Rhs=Self> {  type Output; fn add(self, rhs: Rhs) -> Self::Output;}
```

## Arithmetic Operators

The following table shows the relation between arithmetic operators and traits:

Operator	Trait
a + b 	Add
a - b 	Sub
-a 	Neg
a * b 	Mul
a / b 	Div
a % b 	Rem

> Apart from Rem, which is an abbreviation for Remainder, also known as mod in some other languages, those are pretty obvious. The binary operator traits all have a RHS (=right-hand-side) generic type bound which defaults to Self, as well as an associated Output type that the implementation has to declare.

---

> `tags` [[rhs]] [[DEFAULT_CONCRETE_TYPE]] [[arithmetic]]
