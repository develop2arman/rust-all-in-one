## Newtype Pattern
Using the Newtype Pattern to Implement External Traits on External Types
' thin wrapper around the type' : part of Vec<String> is noticed. struct Wrapper(Vec<String>); 
The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for. Then the wrapper type is local to our crate, and we can implement the trait on the wrapper. Newtype is a term that originates from the Haskell programming language. There is no runtime performance penalty for using this pattern, and **the wrapper type is elided at compile time.**

As an example, let’s say we want to implement Display on Vec<T>, which the **orphan rule** prevents us from doing directly because the Display trait and the Vec<T> type are defined outside our crate. We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value

> `Newtype pattern =  wrapper type = NewPattern = Thin Wrapper` :
> Thin wrapping of an existing type in another struct.we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.

```rust
impl Add<Meters> for Millimeters {type Output = Millimeters;fn add(self, other: Meters) -> Millimeters {}}
```

### Wrapper Type Vs Deref Trait

> The downside of using this technique is that #Wrapper is a new type, so it doesn’t have the methods of the value it’s holding. We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>. *If we wanted the new type to have every method the inner type has, implementing the* #Deref trait.
> If we don’t want the Wrapper type to have all the methods of the inner type—for example, to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

> use wrapper types, which allow more flexibility than what is available by default. These, however, incur costs at runtime to ensure that Rust’s safety guarantees are maintained. Another way to phrase this is that Rust allows programmers to opt in to garbage collection
> To explain the wrapper type strategy, let’s introduce a wrapper type: std:rc::Rc. std:rc::Rc takes a type parameter T and is typically referred to as Rc<T>. Rc<T> reads as “R. C. of T” and stands for “a reference-counted value of type T.” Rc<T> provides shared ownership of T. Shared ownership prevents T from being removed from memory until every owner is removed.

> As indicated by the name, reference counting is used to track valid references. As each reference is created, an internal counter increases by one. When a reference is dropped, the count decreases by one. When the count hits zero, T is also dropped.

> Rc<T> implements Clone. Every call to base.clone() increments an internal counter. Every Drop decrements that counter. When the internal counter reaches zero, the original instance is freed.

## Glossery

> `Thin wrapper around the type` : part of Vec<String> is noticed. struct Wrapper(Vec<String>);       

> `wrapper type` = reference-counted value = shared ownership = track valid references

> `tags` [[new_type]] [[orphan]]
