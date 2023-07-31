
> `tags` [[unit_test]] [[pretty_assertions]]

## Tip
Specify it as [dev-dependencies] and it will only be used for compiling tests, examples, and benchmarks. This way the compile time of cargo build won't be affected!

Also add #[cfg(test)] to your use statements, like this:

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};