[[EXTERN-FFI]]

[[SAFE_ABSTRACTION]]

[[SAFE_STATIC_MUT]]

[[THREAD_UNSAFE]]

---

> An unsafe block implies that the programmer takes full responsibility for any consequences


 > When the compiler tries to determine whether or not code upholds the guarantees, it’s better for it to reject some valid programs than to accept some invalid programs. Although the code might be okay, *if the Rust compiler doesn’t have enough information to be confident, it will reject the code*. In these cases, you can use **unsafe code to tell the compiler, “Trust me, I know what I’m doing.”** Be warned, however, that you use unsafe Rust at your own risk: if you use unsafe code incorrectly, problems can occur due to memory unsafety, such as null pointer dereferencing.
> It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked. The unsafe keyword only gives you access to these five features that are then not checked by the compiler for memory safety. You’ll still get some degree of safety inside of an unsafe block.

>In addition, unsafe does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems: the intent is that as the programmer, you’ll ensure the code inside an unsafe block will access memory in a valid way.

> Keep unsafe blocks small; you’ll be thankful later when you investigate memory bugs.

> To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API,  Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited. **Wrapping unsafe** code in a safe abstraction prevents uses of unsafe from leaking out into all the places that you or your users might want to use the functionality implemented with unsafe code, because using a safe abstraction is safe.

## Dereferencing a Raw Pointer
Different from references and smart pointers, raw pointers:

* Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
* Aren’t guaranteed to point to valid memory
* Are allowed to be null
* Don’t implement any automatic cleanup
* By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.

```rust    
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
```
Notice that we don’t include the unsafe keyword in this code. We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block, as you’ll see in a bit.

We’ve created raw pointers by using as to cast an immutable and a mutable reference into their corresponding raw pointer types. Because we created them directly from references guaranteed to be valid, we know these particular raw pointers are valid, but we can’t make that assumption about just any raw pointer.

To demonstrate this, next we’ll create a raw pointer whose validity we can’t be so certain of. Listing below shows how to create a raw pointer to an arbitrary location in memory. Trying to use arbitrary memory is undefined: there might be data at that address or there might not, the compiler might optimize the code so there is no memory access, or the program might error with a #segmentation_fault. Usually, there is no good reason to write code like this, but it is possible.

```rust
    let address = 0x012345usize;
    let r = address as *const i32;
```
> we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race. Be careful!

```rust,compile_fail,no_run
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2); //datarace
    }
```

## Calling an Unsafe Function or Method

> By calling an unsafe function within an unsafe block, we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts.

```rust,compile_fail,no_run   
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```


## Accessing Fields of a Union

The final action that works only with unsafe is accessing fields of a union. A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because **Rust can’t guarantee the type of the data currently being stored in the union instance.**

This will be laid out equivalently to the following more complex Rust types:


```rust, no_run, compile_fail
// C-compatible layout with a specified discriminant size:
// #[repr(C, u8)]
// A specific integer type (called Int as a shorthand below):
// #[repr(u8)]
// 4 bytes with #[repr(u8)], but would occupy 6 bytes with #[repr(C, u8)], as more padding is required

#[repr(C)]
union TwoCasesRepr {
    A: TwoCasesVariantA,
    B: TwoCasesVariantB,
}
        
#[repr(u8)]
enum TwoCasesTag { A, B }

#[repr(C)]
struct TwoCasesVariantA(TwoCasesTag, u8, u16);

#[repr(C)]
struct TwoCasesVariantB(TwoCasesTag, u16);
```

## When to Use Unsafe Code

Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon. But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety. When you have a reason to use unsafe code, you can do so, and having the explicit unsafe annotation makes it easier to track down the source of problems when they occur.


## More Info

- [Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines)
