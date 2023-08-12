

[[clp-feature-dryoc]]

[[rd-feature-shapes-lib]]

[[rd-macro-fearues]]

[[rd-carg-metadata]]

---

## Feature Rust for what?

To express conditional compilation and optional dependencies.
The ability to pick compile time features in Rust can improve your code's performance, size, maintainability, safety, and portability.
Features for the package being built can be enabled on the command-line with flags such as `--features`

- The `--no-default-features` command-line flag disables the default features of the package.
- The `default-features = false` option can be specified in a dependency declaration.
- `--all-features`: Activates all features of all packages selected on the command-line.


## Caution
- [x] Dependencies automatically enable default features unless `default-features = false` is specified.
- [x] Another issue is that it can be a `SemVer incompatible change` to remove a feature from the default set, so you should be confident that you will keep those features.
> A different feature resolver can be specified with the resolver field in Cargo.toml, like this:
```rust,no_run, compile_fail
[package]
name = "my-package"
version = "1.0.0"
resolver = "2"
```
- [x] Dependencies can be marked `optional`, **which means they will not be compiled by default** except using `--features name` to enable dependency that has `optional` field.
- [x]  There are rare cases where features may be mutually incompatible with one another. consider adding a compile error to detect this scenario. For example:
```rust,no_run, compile_fail
#[cfg(all(feature = "foo", feature = "bar"))]
compile_error!("feature \"foo\" and feature \"bar\" cannot be enabled at the same time");
```
> When there is a conflict, choose one feature over another. The **cfg-if** package can help with writing more complex cfg expressions.

- [x] For example, if you want to optionally support no_std environments, do not use a no_std feature. Instead, use a std feature that enables std. For example:

```rust,no_run, compile_fail
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
pub fn function_that_requires_std() {
    // ...
}
```

## Rules 

```rust,no_run, compile_fail
[dependencies]
ravif = { version = "0.6.3", optional = true }
rgb = { version = "0.8.25", optional = true }

[features]
avif = ["dep:ravif", "dep:rgb"]
```

In above example, the avif feature will enable the two listed dependencies. This also avoids creating the implicit ravif and rgb features, since we don’t want users to enable those individually as they are internal details to our crate.

### Dependency features

Features of dependencies can be enabled within the dependency declaration. The features key indicates which features to enable:

```rust,no_run, compile_fail
[dependencies]
# Enables the `derive` feature of serde.
serde = { version = "1.0.118", features = ["derive"] }
```

The default features can be disabled using default-features = false:

```rust,no_run, compile_fail
[dependencies]
flate2 = { version = "1.0.3", default-features = false, features = ["zlib"] }
```
Features of dependencies can also be enabled in the [features] table. The syntax is "package-name/feature-name". For example:

```rust,no_run, compile_fail
[dependencies]
jpeg-decoder = { version = "0.1.20", default-features = false }

[features]
//Enables parallel processing support by enabling the "rayon" feature of jpeg-decoder.
parallel = ["jpeg-decoder/rayon"]
```

The "package-name/feature-name" syntax will also enable package-name if it is an optional dependency. Often this is not what you want. You can add a ? as in `"package-name?/feature-name"` which will only enable the given feature if something else enables the optional dependency.

> Note: The ? syntax is only available starting with Rust 1.60.

## How Does C/C++ Compare?
C++ does not have a built-in feature directly equivalent to the ability to pick compile time features in Rust. However, C++ does have a number of preprocessor directives that can be used to include or exclude certain code at compile time selectively.


# How to implement?

Code of `#[cfg(feature = "my-feature")]` is used for a function or module. It is feasable to enable a feature flag for a specific struct or enum, a test/benchmark case, and a specific implementation of a trait as well.

To enable a feature only when multiple flags are set, you can use the 

> `#[cfg(all(feature1, feature2, ...))].`

## Example 1

```rust, no_run, compile_fail

// SubFolder monitor mod.rs
//! Signal monitor
#[cfg(unix)]
#[path = "unix.rs"]
mod imp;

#[cfg(windows)]
#[path = "windows.rs"]
mod imp;

#[cfg(not(any(windows, unix)))]
#[path = "other.rs"]
mod imp;

pub use self::imp::create_signal_monitor;
//unix.rs or windows.rs or other.rs
pub fn create_signal_monitor(){
    unimplemented!();
}

```

## Example 2

```rust, no_run, compile_fail
 //Enables use statement only when serde is enabled.
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


## Example 3

```rust, no_run, compile_fail
#[cfg(feature = "shapes")]
pub mod shapes {}
```

### Cargo.toml

```rust, no_run, compile_fail
[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true , default-features = true}
rgb = { version = "0.8.25", features = ["serde"], optional = true , default-features = true}
[features]
default = ["shapes"]
color = ["dep:rgb"]
shapes = ["color", "dep:serde", "rgb?/serde"]
```

---

## Making Featured Everythings

```rust, no_run, compile_fail
//! Memory allocator
#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
#[cfg(feature = "tcmalloc")]
#[global_allocator]
static ALLOC: tcmalloc::TCMalloc = tcmalloc::TCMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[cfg(feature = "snmalloc")]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;
#[cfg(feature = "rpmalloc")]
#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc
```

> In this example, you see how to let your users “layer in” the functionality they need, where you can pick how much deeper you want to go:

```rust, no_run, compile_fail
//! Service launchers
pub mod genkey;
#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "manager")]
pub mod manager;
#[cfg(feature = "server")]
pub mod server;
```

> In the below example, you can use blocks to “artificially” scope in entire pieces of code under a feature:

```rust, no_run, compile_fail
#[cfg(feature = "local-tunnel")]
{
    app = app.arg(
        Arg::new("FORWARD_ADDR")
            .short('f')
            .long("forward-addr")
            .num_args(1)
            .action(ArgAction::Set)
            .requires("LOCAL_ADDR")
            .value_parser(vparser::parse_address)
            .required_if_eq("PROTOCOL", "tunnel")
            .help("Forwarding data directly to this address (for tunnel)"),
    );
}
```