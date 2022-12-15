## Rust
#### Version
Internally, Cargo uses the semver crate[4] for parsing the versions

### Usecases
Hostile environments
Concurrent 
Safe programming
Processing

## Safety

![safetay-control](./assets/images/Screenshot%20from%202022-12-14%2017-53-56.png)

> It guarantees that your program is memory-safe without imposing any runtime costs.

### Goal of Rust: Safety

> Rust programs are free from:

1. **Dangling pointers**— <u>*Live references*</u> to data that has become invalid over the course of the program (see [[rust-in-action-dangling]])
2. **Data races**—The inability to determine how a program will behave from <u>*run to run*</u> because external factors change (see [[rust-in-action-race]])
3. **Buffer overflow**—An attempt to access the 12th element of an <u>*array*</u> with only 6 elements (see listing 1.5)
4. **Iterator invalidation**—An issue caused by something that is iterated over after being <u>*altered midway* through</u>  (see listing 1.6)

### Memory model
RAII, Explicit
