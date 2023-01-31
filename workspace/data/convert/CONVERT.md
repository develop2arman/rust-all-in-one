

[[ria-texttoenum]]

---

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
