
[[ria_trait_display]]

[[edu-partial-eq-trait]]

[[other-partialeq]]

---


# Marker Traits
## ?Size
The Sized trait is a marker trait that represents types whose sizes are **known at compile time**. It is implemented for most types in Rust **except for #unsized_types**. All type parameters have an implicit trait bound of Sized in their definition. We can also specify **optional trait bounds using the ?** operator before a trait, but the ? operator with traits **only works for marker traits** as the time of writing this book. It may be extended to other types in future.

## Borrow & AsRef
These are special traits that carry the notion of able to construct a out of any type.


## To Owned

This trait is meant to be implemented for types that can be converted in to an owned version. For example, the &str type has this trait implemented for String. This means the &str type has a method called to_owned() on it that can convert it in to a String type, which is an owned type.


## From & Into
To convert one type into another, we have the From and Into traits. The interesting part about both of these traits is that we only need to implement the From trait and we get the implementation of the Into trait for free, because of the following impl:

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}
```
## Partial Eq
> comparision like <=>=.
> f32 and f64 types only implement the std::cmp::PartialEq trait, whereas other numeric types also implement std::cmp::Eq
