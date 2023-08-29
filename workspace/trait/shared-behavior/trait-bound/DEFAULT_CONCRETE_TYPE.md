
[[edu-trait-bound]]

[[ms-trait-bound]]

---

> We use generic type parameters, we can specify a **default concrete type for the generic type**. This eliminates the need for implementors of the trait to specify a concrete type if the default type works. You specify a default type when **declaring a generic type with the <PlaceholderType=ConcreteType> syntax**.
> This code should look generally familiar: a trait with one method and an associated type. The new part is **Rhs=Self: this syntax is called default type parameters**. The Rhs generic type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method. 

> *If we don’t specify a concrete type for Rhs when we implement the Add trait, the type of Rhs will default to Self*, which will be the type we’re implementing Add on.

> Rhs is a default to allow **extension of the functionality**.


```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

```

> You’ll use default type parameters in two main ways:
- [x] To extend a type without breaking existing code
- [x] To allow customization in specific cases most users won’t need

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

