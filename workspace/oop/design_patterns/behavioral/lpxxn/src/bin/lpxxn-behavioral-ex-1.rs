#![allow(dead_code, unused_variables)]

/// lpxxn-behavioral-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p lpxxn-behavioral_bin --bin lpxxn-behavioral-ex-1```
///
/// ```cargo doc  --package lpxxn-behavioral_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package lpxxn-behavioral_bin```
///
/// ## What
///`chain_of_responsibility`
///
/// ## How
/// Chain of Responsibility is a behavioral design pattern that lets you pass requests along a chain of handlers.
/// Upon receiving a request, each handler decides either to process the request or to pass it to the next handler in the chain.
///
/// The Handler trait declares a method for building the chain of
/// handlers. It also declares a method for executing a request.
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore


trait Handler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a>;
    fn handle(&self, request: &str);
}
struct AHandler<'a> {
    name: String,
    next: Option<&'a dyn Handler<'a>>,
}
impl<'a> AHandler<'a> {
    fn new(name: String) -> AHandler<'a> {
        AHandler { name, next: None }
    }
}
impl<'a> Handler<'a> for AHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
    fn handle(&self, request: &str) {
        println!("{} handle the request: {}", self.name, request);
        if let Some(v) = &self.next {
            v.handle(request);
        }
    }
}
struct BHandler<'a> {
    next: Option<&'a dyn Handler<'a>>,
}
impl<'a> BHandler<'a> {
    fn new() -> BHandler<'a> {
        BHandler { next: None }
    }
}
impl<'a> Handler<'a> for BHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
    fn handle(&self, request: &str) {
        println!("BHandler handle the request: {}", request);
        if let Some(v) = &self.next {
            v.handle(request);
        }
    }
}
struct Client;
impl<'a> Client {
    fn handle<T: Handler<'a>>(h: &T) {
        h.handle("do something...")
    }
}
fn main() {
    let a1 = AHandler::new("dog".to_string());
    Client::handle(&a1);
    println!();
    let mut b = BHandler::new();
    let mut a2 = AHandler::new("cat".to_string());
    b.set_next(&a1);
    a2.set_next(&b);
    Client::handle(&a2);
}
