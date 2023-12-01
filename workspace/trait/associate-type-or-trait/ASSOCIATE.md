
## Trait Methods

> We can have two kinds of methods within a trait: Associated methods, Instance methods

### Associated methods

> **Associated types might seem like a similar concept to generics.**

*Associated types connect a* **type placeholder** *with a trait such that the trait method definitions can use these placeholder types in their signatures.* The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.

One example of a trait with an associated type is the Iterator trait that the standard library provides. The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over. 

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
       unimplemnted!();
    }
}
```
eq:

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

### Instance methods

> These are **methods that have their first parameter as self**. These are only available on instances of the type that are implementing the trait.  self points to the instance of the type implementing the trait. 
> It can be of three types:

> **self  methods**, which *consume the instance* when called; 

> **&self methods**, which only have *read access* to the instance its members (if any); and 

> **&mut self methods**, which have *mutable access* to its members and can modify them or even replace them with another instance. 

> **For example, the as_ref method from the AsRef trait** in the standard library **is an instance method** that takes &self, and is meant to be implemented by types that can be converted to a reference or a pointer.


### Static methods

> These are methods that are **available directly on the type** implementing the trait and **do not need an instance of the type** to invoke them. There are also known **as static methods** in mainstream languages, **for example, the from_str method from the FromStr trait** in the standard library. *It is implemented* for a String and thus allows you to create a String from a &str by calling String::from_str("foo").



## Associated type traits


> Associated types appear quite a bit in the Rust standard library. They are part of the arithmetic **operations** like + and *. They play a big role in the Iterator trait.
> The advantage of them is that, in the implementation, they allow us to declare the assciated type once and use **Self::Out as the return type** or parameter type in any of the trait methods or functions.

> We have **two kind of asscociate type** for purpose:

> **output and constraints associated type (Ty = T).**

```rust,no_run,compile_fail
trait Foo {
    type Out;
    fn get_value(self) -> Self::Out;
}

```

> For Example:

```rust,no_run,compile_fail
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
//-This syntax seems comparable to that of generics. 
```

> A hypothetical definition of the Iterator trait using generics.

```rust,no_run,compile_fail
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

> **With associated types**, we don’t need to annotate types because we can’t implement a trait on a type multiple times.  At the First Example with the definition that uses associated types, we can only **choose what the type of Item will be once**, because there can only be one impl Iterator for Counter. 

> We don’t have to specify that we want an iterator of u32 values everywhere that we call next on Counter.

> This removes the redundant specification of types, as is the case with generic traits. One of the finest examples of associated type traits is the Iterator trait.

```rust,no_run,compile_fail
impl<T, U> GenericTrait<U> for u32 where U: HasAssocType<Ty = T> {}
```

> The syntax for specifying a **default type for a generic type** is <PlaceholderType=ConcreteType> when declaring the generic type.

> `T` "constrains" by being in an "associated type(Ty = T)" in a bound for type `U` which is itself *a generic parameter(GenericTrait)* constraining the trait.

> For example:

```rust,no_run,compile_fail
// we do not need to write trait Add due to is built-in trait
 pub trait Add<RHS = Self> {
     type Output;
     fn add(self, rhs: RHS) -> Self::Output;
 }

impl<T: Add<T, Output=T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}
```

> Next Example:

```rust,no_run,compile_fail
trait AssociatedType {
    // Associated type declaration
    type Assoc;
}

struct Struct;

struct OtherStruct;

impl AssociatedType for Struct {
    // Associated type definition
    type Assoc = OtherStruct;
}

impl OtherStruct {
    fn new() -> OtherStruct {
        OtherStruct
    }
}

fn main() {
    // Usage of the associated type to refer to OtherStruct as <Struct as AssociatedType>::Assoc
    let _other_struct: OtherStruct = <Struct as AssociatedType>::Assoc::new();
}
```

## Associated Types Container Example

Consider the following example of a Container trait. Notice that the type is available for use in the method signatures:

```rust,no_run,compile_fail
trait Container {
    type E;
    fn empty() -> Self;
    fn insert(&mut self, elem: Self::E);
}
```

In order for a type to implement this trait, it must not only provide implementations for every method, but it must specify the type E. Here's an implementation of Container for the standard library type Vec:

```rust,no_run,compile_fail
impl<T> Container for Vec<T> {
    type E = T;
    fn empty() -> Vec<T> { Vec::new() }
    fn insert(&mut self, x: T) { self.push(x); }
}
```

> Relationship between Bounds and WhereBounds

> In this example:

```rust,no_run,compile_fail
trait Example {
    type Output<T>: Ord where T: Debug;
}
```

> Given a reference to the associated type like <X as Example>::Output<Y>, the associated type itself must be Ord, and the type Y must be Debug.

## Glossery

> `The associated type = placeholder type` :  is named Item'pub trait Iterator {type Item;}'

>  Another example is 'HasAssocType<Ty = T>'  

---

> `tags` [[associate_method]] [[instance_method]] [[associate_trait]] [[rhs]]]
