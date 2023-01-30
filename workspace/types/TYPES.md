[[PRIMITIVE]]

[[STRING]]

[[NUM]]

[[GENERIC]]

[[RETURN]]

[[STRUCT]]

[[ENUM]]

[[RHS]]

---

## Class Vs Struct
![Class Vs Struct](../rust/assets/images/class.JPG)

>  Briefly, functions are regarded as **pure**, meaning their behavior is determined solely by their arguments. Methods are inherently **impure**, given that one of their arguments is effectively a side effect.
> objects sometimes implement static methods, which do not include implicit arguments

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

## Glossery

> In Rust, global variables are called static variables
