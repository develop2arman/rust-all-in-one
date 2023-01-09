
[[rd-feature-shapes-lib]]

[[clp-feature-dryoc]]

---

## Example 1

```
Enables use statement only when serde is enabled.
 #[cfg(feature = "serde")]
 use serde::{Deserialize, Serialize};

 use zeroize::Zeroize;


 // Enables the derive() statement only when serde is enabled.
 #[cfg_attr(
     feature = "serde",
     derive(Serialize, Deserialize, Zeroize, Debug, PartialEq)
 )] // B
 //Enables the derive() statement only when serde is disabled.
 #[cfg_attr(not(feature = "serde"), derive(Zeroize, Debug, PartialEq))]
 #[zeroize(drop)]
 //Message container, for use with unencrypted messages
 pub struct Message(pub Box<InputBase>);
 ````

---

## Example 2

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