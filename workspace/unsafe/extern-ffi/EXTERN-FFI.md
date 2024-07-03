[[rd-ffi]]

---

> Functions declared within extern blocks are always unsafe to call from Rust code.
> The reason is that other languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them, so responsibility falls on the programmer to ensure safety.

>the "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level. 


```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

## No Mangle

The `#[no_mangle]` attribute in Rust is used to instruct the compiler not to mangle the name of the function or item it is applied to. Name mangling is a common technique used by compilers to generate unique names for functions, variables, and other identifiers when they are compiled into machine code. This process helps avoid naming conflicts between different parts of a program or between programs.

By default, Rust applies name mangling to all items (functions, structs, enums, etc.) to ensure that each has a unique identifier in the compiled output. However, there are cases where you might want to control this behavior, especially when interfacing with C libraries or when you need to expose certain Rust functions to be called directly from C code without mangling.

Here's how you can use `#[no_mangle]`:

```rust
#[no_mangle]
pub extern "C" fn my_function() {
    // Function body here
}
```

In this example, `my_function` will not have its name mangled, making it possible to call this function directly from C code. The `extern "C"` part specifies that the function uses the C calling convention, which is necessary for interoperability with C code.

It's important to note that using `#[no_mangle]` should be done judiciously, as it can lead to name clashes if not managed carefully. Additionally, since Rust's name mangling scheme is designed to produce unique names, manually specifying names can potentially introduce conflicts within large projects or across different crates.

For more complex scenarios involving external libraries or specific requirements for ABI compatibility, consider using Rust's Foreign Function Interface (FFI) features along with `#[no_mangle]`. These tools allow Rust to interoperate with code written in other languages, facilitating the integration of Rust into existing systems or applications.


##  Using extern Functions to Call External Code

> Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI). An #FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.
extern "C" block, we list the names and signatures of external functions from another language we want to call. The "C" part defines which application binary interface (ABI) the external function uses: the #ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the C programming language’s ABI.

> We also need to add a #[no_mangle] annotation to tell the Rust compiler not to mangle the name of this function. Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.

> In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C:

> We can also use extern to create an interface that **allows other languages to call Rust functions**. Instead of creating a whole extern block, we add the extern keyword and specify the ABI to use just before the fn keyword for the relevant function. We also need to **add a #[no_mangle] annotation** to tell the Rust compiler not to mangle the name of this function. Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, **we must disable the Rust compiler’s name mangling.**

```rust
#![allow(unused)]
fn main() {
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
}
```

- [Rust FFI](https://cratecode.com/info/rust-ffi)

- [ASM FFI](https://doc.rust-lang.org/rust-by-example/unsafe/asm.html)
 
> `tags` #C #ffi #abi #asm
