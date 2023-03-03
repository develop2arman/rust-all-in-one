[[ASSOCIATE]]

[[INHERITANCE]]

[[SHARED_BEHAVIOR]]

[[SAFEOBJECT]]

[[BUILT_IN]]
---

> We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

> Traits are similar to a feature often called interfaces in other languages, although with some differences.

> What is a trait? A trait is a language feature that is analogous to an interface, protocol, or contract. If you have a background in object-oriented programming, consider a trait to be an abstract base class. If you have a background in functional programming, Rust’s traits are close to Haskell’s type classes

> these also support a form of inheritance that’s common in most object oriented languages. For now, though, the thing to remember is that traits represent common behavior (Or reusable codes like println!)that types opt into via the syntax impl Trait for Type.

> After the method signature, instead of providing an implementation within curly brackets, we use a semicolon

> What does PartialEq do for types? It enables comparisons with the == operator. “Partial” allows for cases where two values that match exactly should not be treated as equal, such as the floating point’s NAN value or SQL’s NULL.  When you see a sentence with the following structure, “...T is Debug...”, what they’re saying is that T implements the Debug trait.

> This interface consists of associated items, which come in three varieties:
	- functions
	- types
	- constants

> All traits define an implicit type parameter Self that refers to "the type that is implementing this interface"

> Trait functions may omit the function body by replacing it with a semicolon. This indicates that the implementation must define the function. If the trait function defines a body, this definition acts as a default for any implementation which does not override it. Similarly, associated constants may omit the equals sign and expression to indicate implementations must define the constant value. Associated types must never define the type, the type may only be specified in an implementation.

> we mentioned that to use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).


> Traits are **private** by default.

> Traits are not usable by themselves and are meant to be implemented by types. Traits have the power to establish **relationships between distinct types**. 
> They are the **backbone** to many language features such as closures, operators, smart pointers, loops, compile-time [[data_race]] checks, and much more.

## Why we need traits?

Example:
```rust,no_run,compile_fail,ignore
impl i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

impl i64 {
    fn double(&self) -> Self {
        self * 2
    }
}

fn main() {
    println!("double 5_i32 == {}", 5_i32.double());
    println!("double 5_i64 == {}", 5_i64.double());
}
```
> Result:
```rust,no_run,compile_fail,ignore
error[E0390]: only a single inherent implementation marked with `#[lang = "i32"]` is allowed for the `i32` primitive
```

NOTE: You may be wondering, why this limitation? We won’t get into these kinds of “why” questions here, but at the time of writing, there is [some material online](https://github.com/Ixrec/rust-orphan-rules) you can read regarding this if you’re curious.

> Answer:

```rust
trait Double {
    fn double(&self) -> Self;//-> i32  is wrong
}

impl Double for i32 {
    fn double(&self) -> Self { // or i32
        self * 2
    }
}

impl Double for i64 {
    fn double(&self) -> Self { // or i32
        self * 2
    }
}

fn main() {
    println!("double 5_i32 == {}", 5_i32.double());
    println!("double 5_i64 == {}", 5_i64.double());
}
```

## Why we need trait bound?

> educative-rust-trait-closure-ex-7.rs

## Marker Traits
> Rust identifies these types with a special trait called Copy. **Copy is known as a marker trait**.
> Traits defined in the **std::marker module** are called marker traits. **These traits don't have any method**, and simply have their declaration with their name with an empty body. Examples from the standard library include Copy, Send, and Sync. They are called marker traits because they are used to simply mark a type as belonging to a particular family for to gain some compile time guarantees. Two such examples from the standard library are the **Send and Sync traits** that are auto-implemented by the language for most types whenever appropriate, and convey which **values are safe to send and share across threads**.


## Blanket Trait

Blanket implementation:
> Any implementation where a type appears uncovered. impl<T> Foo for T, impl<T> Bar<T> for T, impl<T> Bar<Vec<T>> for T, and impl<T> Bar<T> for Vec<T> are considered blanket impls. However, impl<T> Bar<Vec<T>> for Vec<T> is not a blanket impl, as all instances of T which appear in this impl are covered by Vec.

Bound:
> Bounds are constraints on a type or trait. For example, if a bound is placed on the argument a function takes, types passed to that function must abide by that constraint.

> We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. The impl block in the standard library looks similar to this code:

```rust,no_run
impl<T: Display> ToString for T {
    // --snip--
}
```
>Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:
```rust
let s = 3.to_string();
```
> Blanket implementations appear in the documentation for the trait in the “Implementors” section.
---

## Universal function call syntax
*Inherent methods on types are given higher priority than other methods with the same name*. To call a trait method, we can use the Universal Function Call Syntax (**UFCS**).
```rust
trait Driver {
    fn drive(&self) {
        println!("Driver's driving!");
    }
}

struct MyCar;

impl MyCar {
    fn drive(&self) {
        println!("I'm driving!");
    }
}

impl Driver for MyCar {}

fn main() {
    let car = MyCar;
    car.drive();
}
```

## Where

```rust,no_run
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify<T: Summary>(item1: &T, item2: &T) {}

pub fn notify(item: &(impl Summary + Display)) {}

pub fn notify<T: Summary + Display>(item: &T) {}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

> We can use a where clause, like this:
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
```
> This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds.


---

## Generic Implementation 

### Constraits

```rust,no_run,compile_fail
trait Trait{}
trait GenericTrait<T> {}
trait HasAssocType { type Ty; }
struct Struct;
struct GenericStruct<T>(T);
struct ConstGenericStruct<const N: usize>([(); N]);
// T constrains by being an argument to GenericTrait.
impl<T> GenericTrait<T> for i32 { /* ... */ }

// T constrains by being an arguement to GenericStruct
impl<T> Trait for GenericStruct<T> { /* ... */ }

// Likewise, N constrains by being an argument to ConstGenericStruct
impl<const N: usize> Trait for ConstGenericStruct<N> { /* ... */ }

// T constrains by being in an associated type in a bound for type `U` which is
// itself a generic parameter constraining the trait.
impl<T, U> GenericTrait<U> for u32 where U: HasAssocType<Ty = T> { /* ... */ }

// Like previous, except the type is `(U, isize)`. `U` appears inside the type
// that includes `T`, and is not the type itself.
impl<T, U> GenericStruct<U> where (U, isize): HasAssocType<Ty = T> { /* ... */ }
```

### Non-Constraits
 
 The rest of these are errors, since they have type or const parameters that do not constrain.
 `T` does not constrain since it does not appear at all.

```rust,no_run,compile_fail
impl<T> Struct { /* ... */ }
// N does not constrain for the same reason.
impl<const N: usize> Struct { /* ... */ }
// Usage of T inside the implementation does not constrain the impl.
impl<T> Struct {
    fn uses_t(t: &T) { /* ... */ }
}
// T is used as an associated type in the bounds for U, but U does not constrain.
impl<T, U> Struct where U: HasAssocType<Ty = T> { /* ... */ }
// T is used in the bounds, but not as an associated type, so it does not constrain.
impl<T, U> GenericTrait<U> for u32 where U: GenericTrait<T> {}
/* 
Example of an allowed unconstraining lifetime parameter:
*/
impl<'a> Struct {}
/* 
Example of a disallowed unconstraining lifetime parameter:
*/
impl<'a> HasAssocType for Struct {
    type Ty = &'a Struct;
}
```

## Glossery

  > `receiver` : for  exampele of type of Self (i.e. self) implies this 
  
