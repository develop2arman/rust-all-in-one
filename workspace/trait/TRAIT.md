[[ASSOCIATE]]

[[INHERITANCE]]

[[SHARED_BEHAVIOR]]

[[SAFEOBJECT]]

[[NONE_OBJECT_SAFE]]

[[BUILT_IN]]

---

> We can use **traits** to define **shared behavior** in an **abstract** way. We can use **trait bounds** to specify that a **generic type** can be any type that has certain behavior.

> Traits are similar to a feature often called interfaces in other languages, although with some differences.

> What is a trait? A trait is a language feature that is analogous to an **interface, protocol, or contract.** If you have a background in object-oriented programming, consider a trait to be an abstract base class. If you have a background in functional programming, Rust’s traits are close to Haskell’s type classes.

> these also **support a form of inheritance** that’s common in most object oriented languages. For now, though, the thing to remember is that traits represent common behavior (Or **reusable** codes like println!)that types opt into via the syntax impl Trait for Type.

> Traits are **private** by default.

> Traits are not usable by themselves and are meant to be implemented by types. Traits have the power to establish **relationships between distinct types**. 
> They are the **backbone** to many language features such as closures, operators, smart pointers, loops, compile-time [[data_race]] checks, and much more.

> After the method signature, instead of providing an implementation within curly brackets, we use a semicolon

> This interface consists of associated items, which come in three varieties:

- [x] Functions
- [x] Types
- [x] Constants

> What does [[PartialEq]] do for types? It enables comparisons with the == operator. “Partial” allows for cases where two values that match exactly should not be treated as equal, such as the floating point’s NAN value or SQL’s NULL.  When you see a sentence with the following structure, “...T **is** Debug...”, what they’re saying is that T **implements** the Debug trait.

> **All traits define an implicit type parameter Self** that refers to "the type that is implementing this interface"

> Trait functions may omit the function body by replacing it with a semicolon. This indicates that the implementation must define the function. If the trait function defines a body, this definition acts as a default for any implementation which does not override it. Similarly, **associated constants** may omit the equals sign and expression to **indicate implementations** must define the constant value. **Associated types** must never define the type, the type may only be **specified in an implementation.**

> we mentioned that to use traits as **trait objects**, we must **put them behind a pointer**, such as **&dyn Trait or Box<dyn Trait>** (Rc<dyn Trait> would work too).


## Why we need traits?

> Example:

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

```no_run,compile_fail,ignore
error[E0390]: only a single inherent implementation marked with `#[lang = "i32"]` is allowed for the `i32` primitive
```

NOTE: You may be wondering, why this limitation? We won’t get into these kinds of “why” questions here, but at the time of writing, there is [some material online](https://github.com/Ixrec/rust-orphan-rules) you can read regarding this if you’re curious. #orphan_rules

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

## Blanket Trait

`Blanket implementation`:
> Any implementation where a type appears uncovered. impl<T> Foo for T, impl<T> Bar<T> for T, impl<T> Bar<Vec<T>> for T, and impl<T> Bar<T> for Vec<T> are considered blanket impls. However, impl<T> Bar<Vec<T>> for Vec<T> is not a blanket impl, as all instances of T which appear in this impl are covered by Vec.

`Bound`:
> Bounds are constraints on a type or trait. For example, if a bound is placed on the argument a function takes, types passed to that function must abide by that constraint.

> We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. The impl block in the standard library looks similar to this code:

```rust,no_run
impl<T: Display> ToString for T {
    // --snip--
}
```

> Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:

```rust,no_run
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

---

## Use-cases

We've seen a lot of the mechanics and basic use of traits above, but they also wind up playing a few other important roles in Rust: 

`Closures`:

> Somewhat like the ClickCallback trait, closures in Rust are simply particular traits. You can read more about how this works in Huon Wilson's in-depth post on the topic.

`Conditional APIs`:

> Generics make it possible to implement a trait conditionally:

```rust,no_run,compile_fail
    struct Pair<A, B> { first: A, second: B }
    impl<A: Hash, B: Hash> Hash for Pair<A, B> {
        fn hash(&self) -> u64 {
            self.first.hash() ^ self.second.hash()
        }
    }
```

Here, the Pair type implements Hash if, and only if, its components do -- allowing the single Pair type to be used in different contexts, while supporting the largest API available for each context. It's such a common pattern in Rust that there is built-in support for generating certain kinds of "mechanical" implementations automatically:

```rust,no_run,compile_fail
    #[derive(Hash)]
    struct Pair<A, B> { .. }
```

`Extension methods`:

> Traits can be used to extend an existing type (defined elsewhere) with new methods, for convenience, similarly to C#'s extension methods. This falls directly out of the scoping rules for traits: you just define the new methods in a trait, provide an implementation for the type in question, and voila, the method is available.

`Markers`:

> Rust has a handful of "markers" that classify types: Send, Sync, Copy, Sized. These markers are just traits with empty bodies, which can then be used in both generics and trait objects. Markers can be defined in libraries, and they automatically provide #[derive]-style implementations: if all of a types components are Send, for example, so is the type. As we saw before, these markers can be very powerful: the Send marker is how Rust guarantees thread safety.

`Overloading`:

> Rust does not support traditional overloading where the same method is defined with multiple signatures. But traits provide much of the benefit of overloading: if a method is defined generically over a trait, it can be called with any type implementing that trait. Compared to traditional overloading, this has two advantages. First, it means the overloading is less ad hoc: once you understand a trait, you immediately understand the overloading pattern of any APIs using it. Second, it is extensible: you can effectively provide new overloads downstream from a method by providing new trait implementations.

`Operators`

> Rust allows you to overload operators like `+` on your own types. Each of the operators is defined by a corresponding standard library trait, and any type implementing the trait automatically provides the operator as well.

`The point(note)`:

> despite their seeming simplicity, traits are a unifying concept that supports a wide range of use cases and patterns, without having to pile on additional language features.


## Special Traits

[Special Traits&Types](https://doc.rust-lang.org/nightly/reference/special-types-and-traits.html#special-types-and-traits)
  
## Glossery

> `Local trait`: A trait which was defined in the current crate. A trait definition is local or not independent of applied type arguments. Given trait Foo<T, U>, Foo is always local, regardless of the types substituted for T and U.
