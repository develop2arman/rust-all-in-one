/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-trait-ops-rangefull```
///
/// ```cargo doc  --package grammer-dtolnay_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package grammer-dtolnay_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `24`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// 
use std::ops::RangeFull;
trait Trait {
    fn method(&self) -> fn();
}
impl Trait for RangeFull {
    fn method(&self) -> fn() {
        print!("1");
        || print!("3")
    }
}
impl<F: FnOnce() -> T, T> Trait for F {
    fn method(&self) -> fn() {
        print!("2");
        || print!("4")
    }
}
fn main() {
    (|| .. .method())();
}

/*
The two rational possibilities are 1 or 24, depending on how the precedence of || .. .method() is disambiguated.

As || ((..).method()), which is a closure whose body invokes our impl of Trait on RangeFull. In this case main would print 1. It would not print 13 because the fn() returned from (..).method() is never invoked by main.

As (|| ..).method(), which is an invocation of our impl of Trait on FnOnce() -> T where T is inferred to be RangeFull. In this case main would print 24.

The latter of those is the correct answer.

We can achieve the former behavior by explicitly parenthesizing as shown in the bullet above.

Partially parenthesizing as || (.. .method()) is not sufficient. This results in a parse error.

error: expected one of `)` or `,`, found `.`
  --> src/main.rs:22:13
   |
22 |     (|| (.. .method()))();
   |            -^ expected one of `)` or `,`
   |            |
   |            help: missing `,`
Correctly handling a quite ambiguous expression like || .. .method() is a challenge for tooling, as seen by the associated bugs in Rustfmt (rust-lang/rustfmt#4808) and Syn (dtolnay/syn#1019).
*/