#![allow(dead_code, unused_variables)]

/// rust-doc-trait-inheritance-ex-2
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-inheritance_bin --bin rust-doc-trait-inheritance-ex-2```
///
/// ```cargo doc  --package rust-doc-trait-inheritance_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-inheritance_bin ```
///
/// ## What
/// `Subtrait1`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///

// #![allow(unused)]
// fn main() {
// trait Shape { fn area(&self) -> f64; }
// trait Circle : Shape { fn radius(&self) -> f64; }
// }

// #![allow(unused)]
// fn main() {
// trait Shape { fn area(&self) -> f64; }
// trait Circle where Self: Shape { fn radius(&self) -> f64; }
// }

// #![allow(unused)]
// fn main() {
// trait Shape { fn area(&self) -> f64; }
// trait Circle where Self: Shape {
//     fn radius(&self) -> f64 {
//         // A = pi * r^2
//         // so algebraically,
//         // r = sqrt(A / pi)
//         (self.area() /std::f64::consts::PI).sqrt()
//     }
// }
// }


fn main() {
trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }
fn print_area_and_radius<C: Circle>(c: C) {
    // Here we call the area method from the supertrait `Shape` of `Circle`.
    println!("Area: {}", c.area());
    println!("Radius: {}", c.radius());
}
}
