[[ASSOCIATE]]

[[INHERITANCE]]

---

> Traits are private by default.

> Traits are not usable by themselves and are meant to be implemented by types. Traits have the power to establish relationships between distinct types. 
> They are the backbone to many language features such as closures, operators, smart pointers, loops, compile-time data race checks, and much more.

## Trait Methods

> We can have two kinds of methods within a trait: Associated methods, Instance methods

### Associated methods
> These are methods that are available directly on the type implementing the trait and do not need an instance of the type to invoke them. There are also known **as static methods** in mainstream languages, **for example, the from_str method from the FromStr trait** in the standard library. *It is implemented* for a String and thus allows you to create a String from a &str by calling String::from_str("foo").

### Instance methods
> These are **methods that have their first parameter as self**. These are only available on instances of the type that are implementing the trait.  self points to the instance of the type implementing the trait. It can be of three types: self  methods, which consume the instance when called; &self methods, which only have read access to the instance its members (if any); and &mut self methods, which have mutable access to its members and can modify them or even replace them with another instance. **For example, the as_ref method from the AsRef trait** in the standard library is an instance method that takes &self, and is meant to be implemented by types that can be converted to a reference or a pointer.

## Associated type traits
> This removes the redundant specification of types, as is the case with generic traits. One of the finest examples of associated type traits is the Iterator trait.
- [x] The advantage of them is that, in the implementation, they allow us to declare the assciated type once and use **Self::Out as the return type** or parameter type in any of the trait methods or functions.

```rust
trait Foo {
    type Out;
    fn get_value(self) -> Self::Out;
}
```

## Generic traits
> Two such examples are is the **From<T> and Into<T> traits**, which allow from conversion from a type to a type T and vice versa.
> generic traits can get quite **verbose** when they are declared with three or four generic types.

```rust,no_compile
pub trait From<T> {
    fn from(T) -> Self;
}
```

## Trait Inheritance
> Traits can also specify in their declaration that they depend on other traits; this is a feature known as trait inheritance.
> One such example from the standard library is the **Copy trait, which requires  the type to also implement the Clone trait.**

## Marker Traits
> Traits defined in the **std::marker module** are called marker traits. **These traits don't have any method**, and simply have their declaration with their name with an empty body. Examples from the standard library include Copy, Send, and Sync. They are called marker traits because they are used to simply mark a type as belonging to a particular family for to gain some compile time guarantees. Two such examples from the standard library are the **Send and Sync traits** that are auto-implemented by the language for most types whenever appropriate, and convey which **values are safe to send and share across threads**.

---

## Trait bounds

```rust,no_compile
fn load<T: Loadable>(&self, entity: T) { .. }
```

> Notice the : Loadable part. This is how we specify a trait bound. Trait bounds allow us to constrain the range of parameters that a generic API can accept. Specifying a trait bound on a generic item is similar to how we specify types for variables, but here the variable is the generic type T and the type is some trait, such as**T: SomeTrait**. 
> Trait bounds are almost **always needed when defining generic functions**. If one defines a generic function that takes T without any trait bounds, we cannot call any of the methods since Rust does not know what implementation to use for the given methods.
>> The **where** clause decouples the trait bound from the function signature and makes it readable.
> 
> We can specify **trait bounds on types** too:

```rust,no_compile
struct Foo<T: Display> {..}
```
```rust,ignore
struct TEST<T: Display> {..}
```
```rust
fn main(){
    println("Hi amran")
}
```
```rust
    println("Hi amran")
```

```rust
# fn main(){
    println("Hi amran")
# }
```
