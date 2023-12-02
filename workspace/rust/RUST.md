[![codecov](https://codecov.io/gh/arman2develop/rust-all-in-one/graph/badge.svg?token=OXWKHZI4FP)](https://codecov.io/gh/arman2develop/rust-all-in-one)

<!--
https://codecov.io/gh/arman2develop/rust-all-in-one/graphs/tree.svg?token=OXWKHZI4FP
-->

---

# Rust

<details>
  
  * In fact, Rust contains several layers of the Standard Library: `core`, `alloc` and `std`. 
  * `core` includes the most basic types and functions that don't depend on `libc`, allocator or even the presence of an operating system. 
  * `alloc` includes types which require a global heap allocator, such as `Vec`, `Box` and `Arc`.
  * Embedded Rust applications often only use `core`, and sometimes `alloc`.

</details>

## Commands

`time cargo run`

`cargo doc  --workspace --message-format short --no-deps --open --color always`

`cargo test --doc --workspace`

## Use-cases

> Hostile environments: In situations where **safety** is of utmost concern, Rust’s guarantees are a perfect fit.
- Concurrent
- Safe programming
- Processing
- Replacing legacy C or C++
>
> npm chose Rust to handle CPU-bound bottlenecks.

## Syntax
  
  > path`:	::
 
  > `!`: that’s known in type theory lingo as the empty type because it has no values. We prefer to call it the never type.Functions that return never are called diverging functions.

  
  > `?Sized`: T may or may not be Sized-Trait syntax with this meaning is only available for Sized, this notation overrides the default that generic types must have a known size at compile time. not any other traits.
  

## Safety

![safetay-control](./assets/images/safety.png)

> It guarantees that your program is memory-safe without imposing any runtime costs.

### Goal of Rust: Safety

> Rust programs are free from:

1. **Dangling pointers**— <u>_Live references_</u> to data that has become invalid over the course of the program (see [[ria-data-csv-bin]])

2. **Data races**—The inability to determine how a program will behave from <u>_run to run_</u> because external factors change (see [[pnkfx-mpsc]])

3. **Buffer overflow**—An attempt to access the 12th element of an <u>_array_</u> with only 6 elements 

4. **Iterator invalidation**—An issue caused by something that is iterated over after being <u>_altered midway_ through</u> 

## Memory model

### RAII

Rust uses [[RAII]] (resource acquisition is initialization) to keep track of when variables and all their references are in and out of scope. Once they are out of scope, memory can be released. The borrow checker will not allow references to out of scope variables, and it only allows one mutable reference or multiple immutable references, but never both.

### Explicit


### Rust programs have 3 memory 
(regions where data is stored)

#### data memory 

For data that is fixed in size and static (i.e. always available through life of program). Consider the text in your program (e.g. "Hello World!"): This text's bytes are only ever read from one place and therefore can be stored in this region. Compilers make lots of optimizations with this kind of data, and they are generally considered very fast to use since locations are known and fixed.

#### stack memory 
For data that is declared as variables within a function. The location of this memory never changes for the duration of a function call; because of this compilers can optimize code so stack data is very fast to access.

#### heap memory 
For data that is created while the application is running. Data in this region may be added, moved, removed, resized, etc. Because of its dynamic nature it's generally considered slower to use, but it allows for much more creative usages of memory. When data is added to this region we call it an allocation. When data is removed from this section we call it a deallocation. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.


> String struct is also on stack,but holds a reference to data on heap

```rust
  let s= Struct{x:y,z:w}  
```


## Numerous community tools 
(for improving code quality and productivity)

>rust-clippy, an advanced linter and style tool - This warns you of common mistakes and potential **code smells**. Clippy relies on **compiler plugins** that are marked as unstable, so it is available with nightly Rust only. With rustup, you can switch to nightly easily.
>
>rustfmt, an opinionated code formatter-It formats code according to conventions that are mentioned in the Rust style guide.
> rust-analyzer, full-featured IDE integration for the Rust language
> 
> **sccache**, a compiler cache for rustc


## Test

> A test double is the general programming concept for a type used in place of another type during testing. Mock objects are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.

> Rust doesn’t have objects in the same sense as other languages have objects, and Rust doesn’t have mock object functionality built into the standard library as some other languages do. However, you can definitely create a struct that will serve the same purposes as a mock object.

### std:prelude

To understand what is included in local scope by default(like try_into()), you should investigate the std:: [[prelude]] module. Its documentation is available online at [prelude](https://doc.rust-lang.org/std/prelude/index.html)

### no_std

Finally, Rust is very well suited for embedded development and shellcodes. Because these environments don't rely on a proper Operating System, you generally can't use Rust's standard library and you need to use the core library instead.

For these usecases, we use the #![no_std] attribute:

```rust,no_run,compile_fail
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() {
  // ...
}
```
## Method & Func

> Methods are functions that are coupled to some object. From a syntactic point of view, these are just functions that don’t need to specify one of their arguments. Rather than calling open() and passing a File object in as an argument (read(f, buffer)), methods allow the main object to be implicit in the function call (f.read(buffer)) using the dot operator.1

> There are a number of theoretical differences between methods and functions, but a detailed discussion of those computer science topics is available in other books. Briefly, functions are regarded as pure, meaning their behavior is determined solely by their arguments. 

> Methods are inherently impure, given that one of their arguments is effectively a side effect. These are muddy waters, though. Functions are perfectly capable of acting on side effects themselves. Moreover, methods are implemented with functions. And, to add an exception to an exception, objects sometimes implement static methods, which do not include implicit arguments.

> To define methods, Rust programmers use an impl block

## Version

Internally, Cargo uses the **semver** crate for parsing the versions


## Glossery

> `Uninhabited`: type is the never type !, or an enum with no variants enum Never { }. Opposite of Inhabited.

> `Crate`: A crate is the unit of compilation and linking. There are different types of crates, such as libraries or executables. Crates may link and refer to other library crates, called external crates. A crate has a self-contained tree of modules, starting from an unnamed root module called the crate root. Items may be made visible to other crates by marking them as public in the crate root, including through paths of public modules.

> `Module`: A module is a container for zero or more items. Modules are organized in a tree, starting from an unnamed module at the root called the crate root or the root module. Paths may be used to refer to items from other modules, which may be restricted by visibility rules.

> `Name` A name is an identifier or lifetime or loop label that refers to an entity. A name binding is when an entity declaration introduces an identifier or label associated with that entity. Paths, identifiers, and labels are used to refer to an entity.

> `Name resolution`: Name resolution is the compile-time process of tying paths, identifiers, and labels to entity declarations.

> `Namespace`: A namespace is a logical grouping of declared names based on the kind of entity the name refers to. Namespaces allow the occurrence of a name in one namespace to not conflict with the same name in another namespace. Within a namespace, names are organized in a hierarchy, where each level of the hierarchy has its own collection of named entities.

> `Prelude`: Prelude, or The Rust Prelude, is a small collection of items - mostly traits - that are imported into every module of every crate. The traits in the prelude are pervasive.

> `Scope`: A scope is the region of source text where a named entity may be referenced with that name.

---

## Notations 

```rust, compile_fail, no_run
RArrow ->
Eq/EqualArrow =>
; Semi or SemiColon
: Colon
<> Cover, Angle Bracket
[] Square Bracket
() Parenthese
```

---

[Rust Glossary](https://doc.rust-lang.org/nightly/reference/glossary.html)

[Rust Notation](https://doc.rust-lang.org/reference/notation.html)
