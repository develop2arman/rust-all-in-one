/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-display-drop```
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
/// `212`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// **
use std::fmt::{self, Display};
struct S;
impl Display for S {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("1")
    }
}
impl Drop for S {
    fn drop(&mut self) {
        print!("2");
    }
}
fn f() -> S {
    S
}
fn main() {
    let S = f();
    print!("{}", S);
}
/*
No value of type S gets dropped within the body of function f. The function f conjures an S and returns ownership of it to the caller of f; the caller determines when to drop the S of which it received ownership.

On the first line of main, we call f() and perform an infallible match that binds no new variables. As no variables are declared on this line, there is no variable that could be the owner of the S returned by f() so that S is dropped at that point, printing 2. The S in let S = f() is a unit struct pattern (not a variable name) that matches a value of type S via destructuring but does not bind the value to any variable.

The second line of main conjures a new S, prints it, and drops it at the semicolon.
*/