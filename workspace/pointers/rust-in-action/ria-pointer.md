

![Pointers in memory](../../rust/assets/images/pointers1.JPG)

![Memory](../../rust/assets/images/memory.JPG)


## usize

> [[usize]] is the memory address size for the CPU the code is compiled for. That [[CPU]] is called the compile target.

## Raw Pointer
> A [[raw_pointer]] is a memory address without Rust’s standard guarantees. These are inherently **[[unsafe]]**. For example, unlike references (&T), **raw pointers can be null**. If you’ll forgive the syntax, raw pointers are denoted as *const T and *mut T for immutable and mutable raw pointers.

- The difference between a *mut T and a *const T is minimal. These can be freely [[cast]] between one another and tend to be used interchangeably, acting as in-source documentation.

- Rust references (&mut T and &T) compile down to raw pointers. That means that it’s possible to access the performance of raw pointers **without needing to venture into unsafe blocks.**

- Raw pointers do not own their values. The Rust compiler does not check that the referent data is still valid when these are accessed.
  
- Multiple raw pointers to the same data are allowed. Every raw pointer can have write, read-write access to data. This means that there is no time when Rust can guarantee that shared data is valid.


- **It’s unavoidable**. Perhaps some OS call or third-party code requires a raw pointer. Raw pointers are common within C code that provides an external interface. (so because of this we must use raw pointer.)

- **Shared access to something is essential** and runtime performance is paramount. Perhaps multiple components within your application require equal access to some expensive-to-compute variable. If you’re willing to take on the risk of one of those components poisoning every other component with some silly mistake, then raw pointers are an option of last resort. (so because of this we must use raw pointer.)

[[rust-doc]] 

[

- **Are allowed to ignore the borrowing rules** by having both immutable and mutable pointers or multiple mutable pointers to the same location
- **Aren’t guaranteed to point to valid memory**
- Are allowed to be **null**
- **Don’t implement any automatic cleanup**

]



## Raw Poiner In Unsafe [[rust-doc]]

Unsafe Rust has two new types called raw pointers that are similar to references. As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively. 
> **The asterisk isn’t the dereference operator**; it’s part of the type name. In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

> unsafe Notice that we don’t include the unsafe keyword in this code. We can create raw pointers in safe code; we just **can’t dereference raw pointers outside an unsafe block**, as you’ll see in a bit.    

> Creating a raw pointer to an arbitrary memory address

```
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);    
}

let address = 0x012345usize;
let r = address as *const i32;
unsafe {        
    std::ptr::write(r as *mut usize, 0usize); //Memory overwrite to a address
    println!("r1 is: {}", *r);
}
```




---

## Example overview

> [[Cow]] is a [[smart_pointer]] type that reads from its pointer location **without needing to copy**(like Box,  stands for copy on write) it first.
>
> Why write something down when you only need to read it? Perhaps you only want **to make modifications**. This is the role of Cow (copy on write).

```use std::borrow::Cow;```

> [[CStr]] is a C-like string type that allows Rust to read in zero-terminated strings.

```use std::ffi::CStr;```

> #c_char , a type alias for Rust’s i8 type, presents the possibility of a platform-specific nuances.

```use std::os::raw::c_char;```

```
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];

static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
```

```
fn main() {
    let a = 42;
```
> **String is a [[smart_pointer]]** type that holds a pointer to a backing array and a field to store its size

```let b: String;```

> [[Cow]] accepts a type parameter for the data it points to; str is the type returned by CStr. #to_string_lossy (), so it is appropriate here.
> 
> Cow stands for copy on write. This smart pointer type is handy when an external source **provides a buffer**. *Avoiding copies increases runtime performance*.
> 
> std:: [[ffi]] is the **f**oreign **f**unction **i**nterface module from Rust’s standard library. 
> 
> use std::os::raw::c_char; is not strictly needed, but it does make the code’s intent clear.
> 
> C does not define the width of its char type in its standard, although it’s one byte wide in practice. Retrieving the type alias c_char from the std::os:raw module allows for differences.

 ```
 let c: Cow<str>;
 unsafe {
 ```

 > References cannot be [[cast]] directly to *mut T, the type required by String::from_raw_parts(). But [[star_const]] T can be cast to *mut T, leading to this double cast syntax

 ```let b_ptr = &B as *const u8 as *mut u8;```

 > String:: #from_raw_parts () accepts a pointer (*mut T) to an array of bytes, a size, and a #capacity parameter

 ```b = String::from_raw_parts(b_ptr, 10, 10);```

 > Converts a *const u8 to a *const i8, aliased to c_char. The conversion to i8 works because we remain under 128, following the #ASCII standard.

 ```let c_ptr = &C as *const u8 as *const c_char;```

 > Conceptually, CStr:: #from_ptr () takes responsibility for reading the pointer until it reaches 0; then it generates Cow<str> from the result

 ```c = CStr::from_ptr(c_ptr).to_string_lossy();```

 ```
 }
 println!("a: {}, b: {}, c: {}", a, b, c);
 }
 ```

---

You can create pointers safely from any integral value. An i32 is not a Vec<String>, but Rust is quite comfortable .ignoring that here.

```
let ptr = 42 as *const Vec<String>;
```

---

tags #into_raw [[Box]] [[unsafe]] #from_raw_parts #null_mut #write [[ptr]] #size_of #to_string_lossy #from_ptr #CStr
