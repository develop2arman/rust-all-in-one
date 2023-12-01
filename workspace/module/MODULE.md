
## Relative vs Self
By default, Rust recommends absolute imports within root modules. **Absolute** imports are done starting with the **crate keyword**, whereas relative imports are done using the **self keyword**. When re-exporting sub-modules to their parent modules, we might benefit from **relative** imports, *as using absolute imports becomes long and redundant*.

The privacy of items in Rust starts at the module level. As a library author, to expose things to users from a module, we use the pub keyword. But there are items that we only want to expose to other modules within the crate, but not to the users. In such cases, we can use the *-**pub(crate)** modifier for the item, which allows the item to be exposed only within the crate.*

## Dependencies

## Testing a bugfix
Let’s say you’re working with the uuid crate but while you’re working on it you discover a bug. You are, however, quite enterprising so you decide to also try to fix the bug! Originally your manifest will look like:

```rust,no_run,compile_fail
[package]
name = "my-library"
version = "0.1.0"

[dependencies]
uuid = "1.0"
```

First thing we’ll do is to clone the uuid repository locally via:
Next we’ll edit the manifest of my-library to contain:

```rust,no_run,compile_fail
[patch.crates-io]
uuid = { path = "../path/to/uuid" }
$ git clone https://github.com/uuid-rs/uuid.git
```
You’re now building with the local version of uuid (note the path in parentheses in the build output). If you don’t see the local path version getting built then you may need to run 

```rust,no_run,compile_fail
cargo update -p uuid --precise $version 
```

where $version is the version of the locally checked out copy of uuid.

Once you’ve fixed the bug you originally found the next thing you’ll want to do is to likely submit that as a pull request to the uuid crate itself. Once you’ve done this then you can also update the [patch] section. The listing inside of [patch] is just like the [dependencies] section, so once your pull request is merged you could change your path dependency to:

```rust,no_run,compile_fail
[patch.crates-io]
uuid = { git = 'https://github.com/uuid-rs/uuid.git' }
```

## Override
In case the dependency you want to override isn’t loaded from crates.io, you’ll have to change a bit how you use [patch]. For example, if the dependency is a git dependency, you can override it to a local path with:

```rust,no_run,compile_fail
[patch."https://github.com/your/repository"]
my-library = { path = "../my-library/path" }
```
