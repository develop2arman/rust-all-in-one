rust-in-action/ria-types-generic

rd-types-generic

rg-types-generic

pnkfx-types-generic

---

Generics are part of the language design feature that enables code reuse and the Don't repeat yourself (DRY) principle.

the case of statically typed programming languages. They first appeared in ML.

telling the compiler to fill in the actual types later when any code instantiates them.

Generic functions are a cheap way to give the illusion of polymorphic(late binding) code.

Note: By substitution, we mean that every time a generic item is used with a concrete type, a specialized copy of that code is generated at compile time with the type variable T, getting replaced with the concrete type. This process of generating specialized functions with concrete types at compile time is called monomorphization (is doing static_dispatch, early binding) , which is the procedure of doing the opposite of polymorphic functions.

generics is preferred in most cases because it has no runtime overhead, as is the case with trait objects.

we don't need the <Tafter impl because of the presence of u32 as a concrete type.


Let's look at the case of instantiating Vec<T>, a generic type. Without any type signature, the following code does not compile:

```rust,comile_fail,no_run
fn main() {
    let a = Vec::new();
}
```

---

You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won't make your run any slower than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code using generics at compile time. 

Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compile

Every programming language has tools for effectively handling the duplication of concepts.

In Rust, one such tool is generics. Generics are abstract stand-ins for concrete types or other properties. When we’re writing code, we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.
The fragment <T: std::ops::Add<Output = T>says that T must implement std::ops::Add. Using a single type variable T with the trait bound ensures that arguments i and j, as well as the result type, are the same type and that their type supports addition.



`tags` pattern_new_type SAFEOBJECT

 Glossery

`Uncovered/Covered`: T is uncovered, but the T in Vec<Tis covered
