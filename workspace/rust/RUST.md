# Rust

## Commands

`time cargo run`

`cargo doc  --workspace --message-format short --no-deps --open --color always`

`cargo test --doc --workspace`

## Use-cases

Hostile environments: In situations where **safety** is of utmost concern, Rust’s guarantees are a perfect fit.
Concurrent
Safe programming
Processing
Replacing legacy C or C++
> npm chose Rust to handle CPU-bound bottlenecks.

## Safety

![safetay-control](./assets/images/Screenshot%20from%202022-12-14%2017-53-56.png)

> It guarantees that your program is memory-safe without imposing any runtime costs.

### Goal of Rust: Safety

> Rust programs are free from:

1. **Dangling pointers**— <u>_Live references_</u> to data that has become invalid over the course of the program (see [[ria-data-csv-bin]])
2. **Data races**—The inability to determine how a program will behave from <u>_run to run_</u> because external factors change (see [[ria-race]])
3. **Buffer overflow**—An attempt to access the 12th element of an <u>_array_</u> with only 6 elements (see listing 1.5)
4. **Iterator invalidation**—An issue caused by something that is iterated over after being <u>_altered midway_ through</u> (see listing 1.6)

## Memory model

### RAII

Rust uses RAII (resource acquisition is initialization) to keep track of when variables and all their references are in and out of scope. Once they are out of scope, memory can be released. The borrow checker will not allow references to out of scope variables, and it only allows one mutable reference or multiple immutable references, but never both.

### Explicit

## Numerous community tools for improving code quality and productivity:

rust-clippy, an advanced linter and style tool
rustfmt, an opinionated code formatter
**sccache**, a compiler cache for rustc
rust-analyzer, full-featured IDE integration for the Rust language

---

### std:prelude

To understand what is included in local scope by default(like try_into()), you should investigate the std::prelude module. Its documentation is available online at [prelude](https://doc.rust-lang.org/std/prelude/index.html)




### Float Type

[[ria-types-complex]]

## Version

Internally, Cargo uses the **semver** crate for parsing the versions

## Glossery

### Ownership
Ownership has a particular meaning within Rust. An owner is able to make any changes to the data and is responsible for deleting values that it owns when it leaves scope.

### Literal
"hello" called-> string literal, equal=> &'static str.
