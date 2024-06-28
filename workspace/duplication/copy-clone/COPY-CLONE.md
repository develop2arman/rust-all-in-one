
[[ria-copy-clone]]
[[pnkfx-copy-clone]]
[[edu-copy-clone]]
[[SEMANTIC]]
---

# Semantics

Move semantics: A value that gets moved to the receiving item when accessed through a variable or reassigning to a variable exhibits move semantics. Rust has move semantics  by default due to its affine type system. A highlighting part of affine type systems is that values or resources can only be used once, and Rust exhibits this **property with the ownership rule.**

Copy semantics: A value that gets copied (as in a bitwise copy) by default when assigned or accessed through a variable or passed to/returned from a function exhibits copy semantics. **This means that the value can be used any number of times and each value is completely new**.

## Copy

So why do Rust programmers not always use Copy? There are three main reasons:

The Copy trait implies that there will only be negligible performance impact. This is true for numbers but not true for types that are arbitrarily large, such as String.Because Copy creates exact copies, it cannot treat references correctly. 

Naïvely *copying a reference to T would (attempt to) create a second owner of T*. 

That would cause problems later on because there would be multiple attempts to delete T as each reference is deleted.

It appears that Copy depends on the Clone trait. This is because Copy is defined in the standard library as follows:

```rust
pub trait Copy: Clone { }
```
Copy is an auto trait that is implemented automatically on most stack data types such as **primitives and immutable references** but **Types that don't implement Copy are Vec<T>**, String, and mutable references. To make copies of these values, we use the more explicit Clone trait.

Items such as **String and Vec that are heavy to copy**, only implements  the Clone trait. **Smart pointer types also implement the Clone trait** where they just copy the pointer and extra metadata such as the reference count while pointing to the same heap data

## Clone

When to implement Clone on a type:

The Clone trait merely declares a clone method, which needs to be called **explicitly**.

 If your type also contains a value on the heap as part of its representation, then opt for implementing Clone, which makes it **explicit to users** that will also be *cloning the heap data*.

If you are implementing a **smart pointer** type *such as a reference counting type, you should implement Clone on your type to only copy the pointers on the stack*.


## Shallow copy VS Deep copy

Concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow [[copy]] . If a type implements the Copy trait, a variable is still valid after assignment to another variable.

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

### Clone Vs Copy

**Without Copy, Rust applies move semantics to a type’s access**. When using [[Clone]], copying data is explicit.Until a type implements either Copy or Clone, its internal data cannot be copied.

Types can opt into two modes of duplication: cloning and copying.

```no_run,compile_fail
Cloning (std::clone::Clone) 
May be slow and expensive.
Never implicit. A call to the .clone() method is always required. 
May differ from original. Crate authors define what cloning means for their types.
```
Here are some of the types that implement Copy:
- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy.
- For example,** (i32, i32)** implements Copy(because all of [[primitive]] types are fix size so they **store on stack**), but (i32, String) does not.

```no_run,compile_fail
Copying (std::marker::Copy)
Always fast and cheap.
Always implicit.
Always identical. 
Copies are bit-for-bit duplicates of the original value.
```


### Copy Vs Vec Vs Slice
In Rust Copy has a specific meaning of **duplicating bytes without doing any additional bookkeeping**. Vec is fundamentally incompatible with this, because it owns heap-allocated storage, which must have only one and exactly one owner. If it was allowed to be Copy, it'd be unclear which of the copies is the last one to free the storage.

Slice &[] can be Copy, because it doesn't free the storage.

### Handle error of Copy

But if the compiler is telling you to make something Copy, that's just a suggestion. You don't have to do this. You may have just misunderstanding of ownership, and you could avoid the need to copy by reorganizing your code, e.g.

- using shared references
- using a different data structure
- moving values to different structs or scopes.It depends on the situation.

```rust,compile_fail,no_run,ignore
#[derive(Debug)]
struct Foo;

fn main() {
    let a = Foo;

    let closure = move || {
        let b = a;
        b
    };

    println!("{:?}", a); //error
    println!("{:?}", b);//error
    println!("{:?}", closure());//no error
}

```

## Glossery

`implicit`:	encapsulation (like method, copy)
  
---

`tags` [[semantic]] [[move]] [[copy]] [[clone]]
