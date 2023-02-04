[[mr-types-static]]

---

> All access to a static is safe, but there are a number of restrictions on statics:
- The type must have the Sync trait bound to allow thread-safe access.
- Constants cannot refer to statics.

> Statics are generally combined with synchronization primitives for any kind of thread-safe use. They are also used to implement **global locks** and when integrating with C libraries

> Until now, we’ve not talked about global variables, which Rust does support but can be problematic with Rust’s ownership rules. *If two threads are accessing the same mutable global variable, it can cause a* [[data_race]].

> 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly. Accessing an **immutable** static variable is **safe**.

> `Newtype pattern =  wrapper type = NewPattern = Thin Wrapper` :
> Thin wrapping of an existing type in another struct.we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.

```rust
impl Add<Meters> for Millimeters {type Output = Millimeters;fn add(self, other: Meters) -> Millimeters {}}
```
