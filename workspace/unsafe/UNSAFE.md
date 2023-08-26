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

## Creating a Safe Abstraction over Unsafe Code

Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe. In fact, wrapping unsafe code in a safe function is a common abstraction. As an example, let’s study the split_at_mut function from the standard library, which requires some unsafe code. We’ll explore how we might implement it. This safe method is defined on mutable slices: it takes one slice and makes it two by splitting the slice at the index given as an argument. Listing 19-4 shows how to use split_at_mut.

```rust,compile_fail,no_run   

fn main(){

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

} 
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..]) //error
}

```

```compile_fail,no_run   
error[E0499]: cannot borrow `*values` as mutable more than once at a time
```

Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice. Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this. When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.

Listing 19-6 shows how to use an unsafe block, a raw pointer, and some calls to unsafe functions to make the implementation of split_at_mut work.

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

```

> Note that we don’t need to mark the resulting split_at_mut function as unsafe, and we can call this function from safe Rust. We’ve created a safe abstraction to the unsafe code with an implementation of the function that uses unsafe code in a safe way, because it creates only valid pointers from the data this function has access to. Attempting to use values as though it’s a valid slice results in undefined behavior.

```rust
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

```
## Using extern Functions to Call External Code

Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI). An #FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.
extern "C" block, we list the names and signatures of external functions from another language we want to call. The "C" part defines which application binary interface (ABI) the external function uses: the #ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the C programming language’s ABI.

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

> We can also use extern to create an interface that **allows other languages to call Rust functions**. Instead of creating a whole extern block, we add the extern keyword and specify the ABI to use just before the fn keyword for the relevant function. We also need to **add a #[no_mangle] annotation** to tell the Rust compiler not to mangle the name of this function. Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, **we must disable the Rust compiler’s name mangling.**

> In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C: This usage of extern does not require unsafe.

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

```
> `tags` [[split_at_mut]] [[from_raw_parts_mut]] [[as_mut_ptr]] [[raw_pointer]] [[pointer]]


## Accessing or Modifying a Mutable Static Variable

In this book, we’ve not yet talked about global variables, which Rust does support but can be problematic with Rust’s ownership rules. If two threads are **accessing the same mutable global variable, it can cause a data race.** #data_race
In Rust, global variables are called static variables.

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

> Accessing and modifying mutable static variables is unsafe.static variable have a fixed address in memory.

> With mutable data that is globally accessible, it’s difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe. Where possible, it’s preferable to use the concurrency techniques and thread-safe smart pointers.

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

## Implementing an Unsafe Trait

By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.

As an example, recall the Sync and Send marker traits, the compiler implements these traits automatically if our types are composed entirely of Send and Sync types. 

> If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and **we want to mark that type as Send or Sync, we must use unsafe**. Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or **accessed from multiple threads**; therefore, we need to do those checks manually and indicate as such with unsafe.

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
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

[Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines)
