[[BOX]]

[[RC]]

---

## What are Smart Pointers?

> They are called smart because they also have extra metadata and code associated with them that gets executed when they are created or destroyed. Being able to **automatically free the underlying resource** when a smart pointer goes out of scope is one of the major reasons to use smart pointers.
> Much of the smartness in smart pointers comes from two traits, called the **Drop trait and the Deref trait**.


### Drop Mechanism
> When we instantiate any Drop implementing value (any **heap** allocated type), the Rust compiler inserts drop method calls after every end of scope, after compilation. So, we don't need to manually call drop on these instances. This kind of automatic reclamation based on scope is inspired by the **RAII** principle of C++.

### Drerf Mechanism
> smart pointer types often implement the Deref trait, which allows us to use the * dereferencing operator with these types. While **Deref gives you read-only access**, there is also **DerefMut, which can give you a mutable reference** to the underlying type. Deref has the following type signature:

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```
> If defines a single method called Deref that takes self by reference and returns a immutable reference to the underlying type. This combined with the deref coercion feature of Rust, reduces a lot of code that you have to write. Deref coercion is when a type automatically gets converted from one type of reference to some other reference.

## Comparistions of Smart pointers 

> Rc<T> enables multiple owners of the same data; whenever somebody takes a new reference, and decrements it when someone releases a reference. When the counter hits zero, the value is dropped.

> Box<T> The ownership semantics with Box type depends on the wrapped type. If the underlying type is Copy, the Box instance becomes copy, otherwise it **moves by default**.allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime. 

> Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

> Arc<T>: This is for atomic reference counting. This is like the previous type, but with atomicity to guarantee multithread safety.

> Cell<T>: This gives us internal mutability for types that implement the Copy trait. In other words, we gain the possibility to get multiple mutable references to something.

> RefCell<T>: This gives us internal mutability for types, without requiring the Copy trait. It uses runtime locking for safety. Lets us have many immutable borrows or one mutable borrow at any point in time.Box<T> and RefCell<T> have single owners.

> Similar to Rc<T>, RefCell<T> is only for use in [[single_threaded]] scenarios.

> Neither Cell<T> nor RefCell<T> are thread safe (they do not implement #Sync )

> Standard library has other types that provide interior mutability:

>> Such as Cell<T>, which is similar except that instead of giving references to the inner value, the **value is copied** in and out of the Cell<T>.

>> There’s also Mutex<T>, which offers interior mutability that’s safe to use **across threads** [[multi_tread]] scenarios.

---

### Pointer Definations

![Smart-Pointer-1](../../rust/assets/images/smart-pointer-1.JPG)

![Smart-Pointer-2](../../rust/assets/images/smart-pointer-2.JPG)

![Smart-Pointer-3](../../rust/assets/images/smart-pointer-3.JPG)

### Box

> The Box type can be used in the following situations:

> It can be used to create **recursive type** definitions. 

> When you need to store types as trait objects.

> When you need to store functions in a collection.


## Other Smart Pointers

> [[Cow]] is a [[smart_pointer]] type that reads from its pointer location **without needing to copy**(like Box,  stands for copy on write) it first.
>
> Why write something down when you only need to read it? Perhaps you only want **to make modifications**. This is the role of Cow (copy on write).

```use std::borrow::Cow;```

> [[CStr]] is a C-like string type that allows Rust to read in zero-terminated strings.

```use std::ffi::CStr;```

> [[c_char]] , a type alias for Rust’s i8 type, presents the possibility of a platform-specific nuances.

```use std::os::raw::c_char;```

```rust
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];

static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
```

```rust
fn main() {
    let a = 42;
}
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

 ```rust,compile_fail,no_run
 let c: Cow<str>;
 unsafe {}
 ```

 > References cannot be [[cast]] directly to *mut T, the type required by String::from_raw_parts(). But [[star_const]] T can be cast to *mut T, leading to this double cast syntax

 ```let b_ptr = &B as *const u8 as *mut u8;```

 > String:: #from_raw_parts () accepts a pointer (*mut T) to an array of bytes, a size, and a [[capacity]] parameter

 ```b = String::from_raw_parts(b_ptr, 10, 10);```

 > Converts a *const u8 to a *const i8, aliased to c_char. The conversion to i8 works because we remain under 128, following the #ASCII standard.

 ```let c_ptr = &C as *const u8 as *const c_char;```

 > Conceptually, CStr:: #from_ptr () takes responsibility for reading the pointer until it reaches 0; then it generates Cow<str> from the result

 ```c = CStr::from_ptr(c_ptr).to_string_lossy();```

 ```rust,compile_fail,no_run

 println!("a: {}, b: {}, c: {}", a, b, c);

 ```
