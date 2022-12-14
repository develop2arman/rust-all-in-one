
## Conditional compilation

### Example

```
#[cfg(feature = "shapes")]
pub mod shapes {}
```
### Cargo.toml

```
[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true , default-features = true}
rgb = { version = "0.8.25", features = ["serde"], optional = true , default-features = true}
[features]
default = ["shapes"]
color = ["dep:rgb"]
shapes = ["color", "dep:serde", "rgb?/serde"]
```
