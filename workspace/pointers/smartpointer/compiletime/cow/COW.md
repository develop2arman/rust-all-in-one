
> [[Cow]] is a [[smart_pointer]] type that reads from its pointer location **without needing to copy**(like Box,  stands for copy on write) it first.
>
> Why write something down when you only need to read it? Perhaps you only want **to make modifications**. This is the role of Cow (copy on write).



```rust
use std::borrow::Cow;
pub enum Cow<'a, B> where B: 'a + ToOwned + 'a + ?Sized,  {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

> First, we have the two variants:
- Borrowed that represents the borrowed version of some type B. This B has to implement the ToOwned trait.
- There is also owned variant which contains the owned version of the type.

> This type is suitable for cases where one needs to avoid allocations where it's not needed. A real world example is the JSON parser crate called serde_json.
---

> [[CStr]] is a C-like string type that allows Rust to read in zero-terminated strings.

```use std::ffi::CStr;```

> [[c_char]] , a type alias for Rust’s i8 type, presents the possibility of a platform-specific nuances.

```use std::os::raw::c_char;```

```rust
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];

static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
```

```rust
fn main() {
    let a = 42;
}
```
> **String is a [[smart_pointer]]** type that holds a pointer to a backing array and a field to store its size

```let b: String;```

> [[Cow]] accepts a type parameter for the data it points to; str is the type returned by CStr. #to_string_lossy (), so it is appropriate here.
> 
> Cow stands for copy on write. This smart pointer type is handy when an external source **provides a buffer**. *Avoiding copies increases runtime performance*.
> 
> std:: [[ffi]] is the **f**oreign **f**unction **i**nterface module from Rust’s standard library. 
> 
> use std::os::raw::c_char; is not strictly needed, but it does make the code’s intent clear.
> 
> C does not define the width of its char type in its standard, although it’s one byte wide in practice. Retrieving the type alias c_char from the std::os:raw module allows for differences.

 ```rust,compile_fail,no_run
 let c: Cow<str>;
 unsafe {}
 ```

 > References cannot be [[cast]] directly to *mut T, the type required by String::from_raw_parts(). But [[star_const]] T can be cast to *mut T, leading to this double cast syntax

 ```let b_ptr = &B as *const u8 as *mut u8;```

 > String:: #from_raw_parts () accepts a pointer (*mut T) to an array of bytes, a size, and a [[capacity]] parameter

 ```b = String::from_raw_parts(b_ptr, 10, 10);```

 > Converts a *const u8 to a *const i8, aliased to c_char. The conversion to i8 works because we remain under 128, following the #ASCII standard.

 ```let c_ptr = &C as *const u8 as *const c_char;```

 > Conceptually, CStr:: #from_ptr () takes responsibility for reading the pointer until it reaches 0; then it generates Cow<str> from the result

 ```c = CStr::from_ptr(c_ptr).to_string_lossy();```

 ```rust,compile_fail,no_run

 println!("a: {}, b: {}, c: {}", a, b, c);

 ```


---

> `tags` [[Box]] [[LinkedList]] [[Cow]] [[from_raw_parts]] [[from_ptr]] [[CStr]] [[raw_pointer]]
