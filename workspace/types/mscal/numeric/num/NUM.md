[[o-bigint]]

[[o-bigrational]]

[[ria-float]]

[[ria-types-complex]]

---

> A collection of numeric types and traits for Rust.
> This includes new types for big integers, rationals, and complex numbers, new traits for generic programming on numeric properties like Integer, and generic range iterators.

> We have some library for generic numbers that say `rug` has a better performance than `num`:

Rug provides integers and floating-point numbers with arbitrary precision and correct rounding:

- Integer is a bignum integer with arbitrary precision,
- Rational is a bignum rational number with arbitrary precision,
- Float is a multi-precision floating-point number with correct rounding, and
- Complex is a multi-precision complex number with correct rounding.
