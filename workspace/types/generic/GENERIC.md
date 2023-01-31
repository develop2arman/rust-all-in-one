[[rust-in-action/ria-types-generic]]

[[rd-types-generic]]

---

> Generics are part of the language design feature that enables code reuse and the **Don't repeat yourself (#DRY) principle.**

> the case of statically typed programming languages. They first appeared in ML.

> telling the compiler to fill in the actual types later when any code instantiates them.

> Generic functions are a cheap way to give the illusion of **polymorphic code**.

> Note: By substitution, we mean that every time a generic item is used with a concrete type, a specialized copy of that code is generated at compile time with the type variable T, getting replaced with the concrete type. This process of generating specialized functions with concrete types at **compile time is called** [[monomorphization]] , which is the procedure of doing the **opposite of** [[polymorphic]] functions.

> generics is preferred in most cases because it has **no runtime overhead**, as is the case with trait objects.

> we don't need the <T> after impl because of the presence of u32 as a concrete type.


>Let's look at the case of instantiating Vec<T>, a generic type. Without any type signature, the following code does not compile:

```rust,comile_fail,no_run
fn main() {
    let a = Vec::new();
}
```

## Right Hand Side

> or default type parameters, for example the fragment <T: std::ops::Add<Output = T>> says that T must implement trait std::ops::Add. Using a single type variable T with the trait bound ensures that arguments i and j, as well as the result type, are the same type and that their type supports addition.

```rust
trait Add<Rhs=Self> {  type Output; fn add(self, rhs: Rhs) -> Self::Output;}
```



## Glossery

### Thin Wrapper


> `Thin wrapper around the type` : part of Vec<String> is noticed. struct Wrapper(Vec<String>);       

> `Newtype pattern =  wrapper type = NewPattern` :
> Thin wrapping of an existing type in another struct.we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.

```rust
impl Add<Meters> for Millimeters {type Output = Millimeters;fn add(self, other: Meters) -> Millimeters {}}
```
---

> `tags` [[pattern_new_type]]
