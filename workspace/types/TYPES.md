[[COMPOUND]]

[[SCALAR]]

[[GENERIC]]

[[RETURN]]

[[RHS]]

[[STATIC]]

[[WRAPPER]]

[[VAR]]


---

## Unit Type

```rust
fn main() {
    let x: () = ();
    let y: () = println!("Hello, world!");
    assert_eq!(x, y);
    println!("All units are the same!");
}
```
## Class Vs Struct
![Class Vs Struct](../rust/assets/images/class.JPG)

>  Briefly, functions are regarded as **pure**, meaning their behavior is determined solely by their arguments. Methods are inherently **impure**, given that one of their arguments is effectively a side effect.
> objects sometimes implement static methods, which do not include implicit arguments

## Const Vs Static Vs Let 
> **Constants** represent concrete values and **don't have any memory location** associated with them. They are inlined wherever they are used.

> **Static** **a fixed memory location** and exist as a single instance in the whole program. These can also be made **mutable**.

> It can be confusing whether or not you should use a constant item or a static item. Constants should, in general, be preferred over statics unless one of the following are true:
- Large amounts of data are being stored
- The single-address property of statics is required.
- Interior mutability is required.
>> Another difference between constants and static variables is that static variables can be mutable.
>> Accessing and modifying mutable static variables is unsafe

> If variables defined with let are immutable, then why does Rust include a const keyword? The short answer is that data behind let can change. Rust allows types to have an apparently contradictory property of interior mutability.

> Some types such as std:sync::Arc and std:rc::[[Rc]] present an immutable façade, yet change their internal state over time. In the case of those two types, these increment a reference count as references to those are made and decrement that count when those references expire.

> At the level of the compiler, let relates more to aliasing than immutability. Aliasing in [[compiler]] terminology refers to having multiple references to the same location in memory at the same time. Read-only references (borrows) to variables declared with let can alias the same data. Read-write references (mutable borrows) are guaranteed to never alias data.

> Generally, if you don't need to rely on #singleton property of statics and its predefined memory location and just want a concrete value, you should prefer using consts. They allow the compiler to make better optimizations and are more straightforward to use.

## Const in Trait, Enum, Struct
```rust
trait Circular {
    const PI: f64 = 3.14;
    //...
}
```

```rust
enum Item {
    One,
    Two
}

struct Food {
    Cake,
    Chocolate
}

impl Item {
    const DEFAULT_COUNT: u32 = 34;
}
impl Food {
    const FAVORITE_FOOD: &str = "Cake";
}
```

## Include_Consequences
> Master Rust says:
> Vec=>String=>&str=>slice=>[u8]

## Type inference
a component of the compiler called the type checker uses the #Hindley_Milner **type inference algorithm to decide** what the types of local variables should be. It is a set of rules about establishing types of expressions based on their usage. As such, it can infer types based on the environment and the way a type is used. One such example is the following:

```rust
let mut v = vec![];
v.push(2);    // can figure type of `v` now to be of Vec<i32>
```
With only the first line initializing the vector, Rust's type checker is unsure of what the type for v should be.
It's only when it reaches the next line, v.push(2), that **it knows that v is of the type, Vec<i32>**. Now the type of v is frozen to Vec<i32>.

> But sometimes, the type checker cannot figure out types of variables in complex situations

```rust
let bytes = file.bytes().collect();//compiler error means Hindley_Milner could not detect type
let bytes: Vec<Result<u8, _>> = file.bytes().collect(); // we need to help algo to detect type
```

## Type Aliases
if you have an API from your crate where you return a Result type, wrapping a complex object as depicted below:

```rust,no_run
pub struct ParsedPayload<T> {
    inner: T
}

pub struct ParseError<E> {
    inner: E
}

pub fn parse_payload<T, E>(stream: &[u8]) -> Result<ParsedPayload<T>, ParseError<E>> {
    unimplemented!();
}

fn main() {
    // todo
}
```
> aliasing
Provides a thin wrapper over Rc<RefCell<T>> for higher ergonomics, as managing nested types gets annoying.

```rust,no_run
// added a type alias
type ParserResult<T, E> = Result<ParsedPayload<T>, ParseError<E>>;

// and modify parse_payload function as:
pub fn parse_payload<T, E>(stream: &[u8]) -> ParserResult<T, E> {
    unimplemented!();
}
```
> another example
```rust,no_run
type SomethingComplex<T> = Vec<Result<Option<T>>>;
```
## Concept Nan

> Floating-point types include “not a number” values (represented in Rust syntax as NAN values) to handle these cases.
> NAN values poison other numbers. 
> Almost all operations interacting with NAN return NAN.Another thing to be mindful of is that, by definition, NAN values are never equal. 
> To program defensively, make use of the **is_nan() and is_finite()** methods. Inducing a crash, rather than silently proceeding with a mathematical error, allows you to debug close to what has caused the problem. The following illustrates using the is_finite().

## Glossery

> In Rust, global variables are called static variables
