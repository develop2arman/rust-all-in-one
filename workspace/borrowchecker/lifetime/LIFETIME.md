
[[LIFETIME_DANGLING]]

[[LIFETIME_ELISION]]

[[LIFETIME_STATIC]]

[[LIFETIME_GENERIC]]

---

# Lifetime

> *lifetime = timetolive = subset of their scope.*

Rust’s lifetime system is a remarkable feature that enforces **memory safety without the need for a garbage collector**. Lifetimes track how long references to data are valid, preventing dangling pointers and memory leaks.
A function’s local variables live until the function returns, while global variables might live for the life of the program.

Make hypotheses about whether or not your experiments will pass the borrow checker before you compile reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, **lifetimes are implicit and inferred**, just like most of the time, types are inferred.

We must annotate types when multiple types are possible. 

In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. **The main aim of lifetimes is to prevent dangling references**, which cause a program to reference data other than the data it’s intended to reference.

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow= **memory-safe operations** and disallow operations that would **create dangling pointers or otherwise violate memory safety.**

For data allocated on the stack, we can easily reason by looking at the code and figure out whether a variable is alive or not. For heap allocated values, though, this isn't clear.

The following are the rules that are followed when eliding lifetimes:
If the input lifetime contains only a single reference, the output lifetime is assumed to be the same

For methods involving self and &mut self, the input lifetime is inferred for the  &self parameter

where we have to specify lifetimes when Rust cannot figure them out for us:

- [x] Function signatures
- [x] Structs and struct fields
- [x] impl blocks

---

Make hypotheses about whether or not your experiments will pass the borrow checker before you compile reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.

All references in Rust have a lifetime, even if they are not explicitly annotated. The compiler is capable of implicitly assigning lifetimes. 

A value’s lifetime is the period when accessing that value is valid behavior. A function’s local variables live until the function returns, while global variables might live for the life of the program.

the notion of ownership is rather limited. An owner cleans up when its values’ lifetimes end.

<'a, 'bdeclares two lifetime variables, 'a and 'b, within the scope of j: &'b i32 binds the lifetime variable 'b to the lifetime of j. The syntax reads as “parameter j is a reference to an i32 with lifetime b.”

All values bound to a given lifetime must live as long as the last access to any value bound to that lifetime.
 No lifetime annotations are required when calling a function.
 
'a in generic func means: function will live at least as long as lifetime 'a
 e.g. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.

- Lifetime annotations don’t change how long any of the references live. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.
- Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
- The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.
- Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

 
<'a, 'bdeclares two lifetime variables, 'a and 'b, within the scope of
j: &'b i32 binds the lifetime variable 'b to the lifetime of j. The syntax reads as “parameter j is a reference to an i32 with lifetime b.”

Although every parameter has a lifetime, these checks are typically invisible as the compiler can infer most lifetimes by itself

All values bound to a given lifetime must live as long as the last access to any value bound to that lifetime.
No lifetime annotations are required when calling a function.

```rust
fn longest_common_prefix<'a>(x: &'a str, y: &'a str) -> &'a str {
    let min_length = std::cmp::min(x.len(), y.len());
    let bytes_x = x.as_bytes();
    let bytes_y = y.as_bytes();
    for i in 0..min_length {
        if bytes_x[i] != bytes_y[i] {
        return &x[..i];
        }
    }
        &x[..min_length]
}

fn main() {
    let string1 = "abc";
    let result;
    {
    let string2 = "abdef";
    result = longest_common_prefix(string1, string2);
    }
    println!("The longest common prefix is: {}", result);
}
// $ rustc lifetimes.rs && ./lifetimes
// Output: The longest common prefix is: ab
```

If we remove lifetimes we'll be encounter an error:

`doesn't have a size known at compile-time`
`help: function arguments must have a statically known size, borrowed types always have a known size`

---

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
 
 Using two lifetime parameters (a and b) indicates that the lifetimes of i and j are decoupled.
 
```rust,compile_fail,no_run
 fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -i32 {}
```

 lifetime of that usage: 
 the LOC('existence time' or Line of code) between when a location is first used in a certain way, and when that usage stops.

 lifetime of that value:
 the LOC (or actual time) between when a value is created, and when that value is dropped.

might be useful when discussing open file descriptors, but also irrelevant here.

`let r: &'c S = &c;`

the 'c part, like a type, also guards what is allowed into r.

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.



## lifetime of that usage

the #LOC ('existence time' or Line of code) between when a location is 
first used in a certain way, and when that usage stops.

## lifetime of that value

the LOC (or actual time) between when 
a value is created, and when that value is dropped.
might be useful when discussing open file descriptors.

## Drop

When values go out of scope or their lifetimes end for some other reason, their **destructors** are called. A destructor is a function that removes traces of the value from the program by deleting references and freeing memory. You won’t find a call to any destructors in most Rust code. The compiler injects that code itself as part of the process of tracking every value’s lifetime.

To provide a **custom destructor** for a type, we implement Drop. This typically is needed in cases where we have used unsafe blocks to allocate memory. Drop has one method, **drop**(&mut self), that you can use to conduct any necessary wind-up activities.

Unfortunately, it’s not straightforward to disable the automatic drop functionality. Disabling drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of automatically. Occasionally, however, you might want to clean up a value early. 
Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the **std::mem::drop function**


### Let Vs Const

#### Scope
const variables are declared in global and local scope unlike let variables that are declared only in the local scope.
Mutability
const variable cannot be mutable unlike let which can be made mutable using mut keyword.
#### Data Type
Unlike let variables, it is mandatory to define the data type of const variables.
#### Set Value at Run-time
The value of const variable can only be set before running the program whereas the let variable can store the result at runtime.
#### Shadowing
Unlike let variables, const variables cannot be shadowed.

---

If variables defined with #let are immutable, then why does Rust include a #const keyword?
 
The short answer is that data behind let can change. Rust allows types to have an apparently contradictory property of interior mutability.
At the level of the [[COMPILER]], let relates more to #alias ing than immutability.
Aliasing in compiler terminology refers to having multiple references **to the same location in memory at the same time** 

**Read-only references** (borrows) to variables declared with **let can alias the same data**.

**Read-write references** (mutable borrows) are guaranteed to **never alias data.**
Some types such as std:sync::Arc and std:#rc::Rc present an immutable façade, yet change their internal state over time. In the case of those two types, these increment a #reference_count as references to those are made and decrement that count when those references expire.


### Const Vs Static

```
const WORDS: &'static str = "hello rust!";
```

Thanks to static lifetime elision, you usually don't have to explicitly use 'static:

```
const WORDS: &str = "hello convenience!";
```

const items looks remarkably similar to static items, which introduces some confusion as to which one should be used at which times. To put it simply,  **constants are inlined**  wherever they're used, making using them identical to simply replacing the name of the const with its value. Static variables, on the other hand, point to **a single location** in memory, which all accesses share. This means that, unlike with constants, they can't have **destructors**, and act as a single value across the  **entire codebase** .

---

When values go out of scope or their lifetimes end for some other reason, their destructors are called. A destructor is a function that removes traces of the value from the program by deleting references and freeing memory. You won’t find a call to any destructors in most Rust code. The compiler injects that code itself as part of the process of tracking every value’s lifetime.

To provide a custom destructor for a type, we implement Drop. This typically is needed in cases where we have used unsafe blocks to allocate memory. Drop has one method, drop(&mut self), that you can use to conduct any necessary wind-up activities.



## Syntax

syntax <'a, 'bdeclares two lifetime variables, 'a and 'b, within the scope of
    j: &'b i32 binds the lifetime variable 'b to the lifetime of j. The syntax reads as “parameter j is a reference to an i32 with lifetime b.”
    Although every parameter has a lifetime, these checks are typically invisible as the compiler can infer most lifetimes by itself.

---

All values bound to a given lifetime must live as long as the last access to any value bound to that lifetime.
No lifetime annotations are required when calling a function.
`Example`
 'a in generic func means: function will live at least as long as lifetime 'a
  e.g. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.
 
---

Using two lifetime parameters (a and b) indicates that the lifetimes of i and j are decoupled.

```rust,no_run,compile_fail
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -i32 {}//working fine without error
```

---


## Glossery

`'a`:	lifetime a,  lifetime=timetolive=subset of their scope, &'a mut i32 // a mutable reference with an explicit lifetime.
