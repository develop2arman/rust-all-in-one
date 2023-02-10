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


## MANGLE
> We also need to add a #[no_mangle] annotation to tell the Rust compiler not to mangle the name of this function. Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.

> In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C:


```rust
#![allow(unused)]
fn main() {
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
}
```

> `tags` 
