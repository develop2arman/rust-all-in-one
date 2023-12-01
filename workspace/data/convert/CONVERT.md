[[AS]]

[[ria-texttoenum]]

---

> Rust provides a trait named Into, which provides compile-time safe type conversions. For example, we can convert from a u32 to a u64 using the following code:
```rust
let y = u32::max_value(); 
let z : u64 = y.into();
```
> Rust’s Into trait resolves the problem of potentially impossible conversions by not implementing them.

> The inverse of the example—converting a u64 to a u32—is impossible with Into. If we try let z : u32 = (12_u64).into(), the into() function call will fail to compile. For conversions that may be possible, Rust provides another trait which is TryInto. The following code uses try_into() to attempt to convert between u64 and u32:

```rust
use std::convert::TryInto;
let z : u32 = (5000_u64).try_into().expect("Conversion error");
```

> Using type casts carelessly will cause your program to behave unexpectedly. For example, the expression 300_i32 as i8 returns 44. 
- *BecauseOf(-): *BecauseOf(?): using as //300-128=172-128=44

> try_into is better than 'as' because of error handling.
> The try_into() method returns an i32 value wrapped within a Result.

> unwrap: if this result does not emit an error. it will resolve with the value. unwrap in not suit for prod because input validation
> The unwrap() method can handle the success value and returns the value of b as an i32 here
---

> `tags` [[try_into]]

## Glossery

 > 'turbofish':	::<>()  , ::. Combined with the (angular brackets=Bound) for generics
