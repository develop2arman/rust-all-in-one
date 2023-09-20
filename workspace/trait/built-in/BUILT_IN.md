[[ria_trait_display]]

[[edu-partial-eq-trait]]

[[other-partialeq]]

---


# Marker Traits
## ?Size

> Sized trait is a **marker trait** that represents types whose **sizes are known at compile time**. It is implemented for most types in Rust **except for #unsized_types**. All type parameters have an implicit trait bound of Sized in their definition. 

> We can also specify **optional trait bounds using the ?** operator before a trait, but the ? operator with traits **only works for marker traits** as the time of writing this book. It may be extended to other types in future.

## Borrow & AsRef
These are special traits that carry the notion of able to construct a out of any type.


## To Owned

This trait is meant to be implemented for types that can be converted in to an owned version. For example, the &str type has this trait implemented for String. This means the **&str type has a method called to_owned() on it that can convert it in to a String type**, which is an owned type.


## From & Into

> For obvious reasons, those traits cannot be auto-derived, but writing them should be trivial in most cases

> To convert one type into another, we have the From and Into traits. The interesting part about both of these traits is that we only need to implement the From trait and we get the implementation of the Into trait for free, because of the following impl:

```rust,no_run,compile_fail
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}
```

Why not implement From everywhere? The orphan rule unfortunately forbids implementing From for types not defined in other crates. For example, I have an Optioned<T> type, that I may want to convert into an Option<T>. Trying to implement From:

```rust,no_run,compile_fail
impl<T: Noned + Copy> From<Optioned<T>> for Option<T> {
    #[inline]
    fn from(self) -> Option<T> { self.map_or_else(|| none(), wrap) }
}
```

I get an error: type parameter T must be used as the type parameter for some local type (e.g. MyStruct<T>); only traits defined in the current crate can be implemented for a type parameter [E0210]

Note that you can implement From and Into with multiple classes, you can have a From<Foo> and a From<Bar> for the same type.

There are a good number of traits starting with Into – IntoIterator, which is stable and which we already have discussed above, just being one of them. There also is FromIterator, which does the reverse, namely constructing a value of your type from an iterator of items.

Then there is FromStr for any types that can be parsed from a string, which is very useful for types that you want read from any textual source, e.g. configuration or user input. Note that its interface differs from From<&str> in that it returns a Result, and thus allows to relate parsing errors to the caller.

## Partial Eq
> comparision like <=>=.

> **f32/64** types only implement the std::cmp::**PartialEq** trait, whereas **other numeric** types also implement std::cmp::**Eq**


## Termination

```rust,no_run,compile_fail, ignore
    ()
    Result<T, E> where T: Termination, E: Debug
    !
```

> For Example:

```rust,no_run,compile_fail
fn main() -> impl std::process::Termination // () or !
{
    std::process::ExitCode::SUCCESS
}
```
