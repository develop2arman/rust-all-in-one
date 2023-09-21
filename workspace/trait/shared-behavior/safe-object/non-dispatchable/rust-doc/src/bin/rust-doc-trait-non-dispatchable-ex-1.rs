#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-non-dispatchable-ex-1```
///
/// ```cargo doc  --package rust-doc-shared-behavior-nondispatchable_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-shared-behavior-nondispatchable_bin ```
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
/// `TODO`
///
/// ## Example
///  `TODO`

fn main() {
use std::rc::Rc;
use std::sync::Arc;
use std::pin::Pin;
// Examples of object safe methods.
trait TraitMethods {
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested_pin(self: Pin<Arc<Self>>) {}
}
struct S;
impl TraitMethods for S {}
let t: Box<dyn TraitMethods> = Box::new(S);
//t.callable since object safe dispatchable
}
