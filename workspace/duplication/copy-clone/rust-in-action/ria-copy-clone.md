
## Shallow copy VS Deep copy

> Concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow [[copy]] . If a type implements the Copy trait, a variable is still valid after assignment to another variable.

> If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

## Clone Vs Copy

> **Without Copy, Rust applies move semantics to a type’s access**. When using [[Clone]], copying data is explicit.Until a type implements either Copy or Clone, its internal data cannot be copied.

> Types can opt into two modes of duplication: cloning and copying.

```
Cloning (std::clone::Clone) 
May be slow and expensive.
Never implicit. A call to the .clone() method is always required. 
May differ from original. Crate authors define what cloning means for their types.
```
> Here are some of the types that implement Copy:
- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy.
- For example,** (i32, i32)** implements Copy(because all of [[primitive]] types are fix size so they **store on stack**), but (i32, String) does not.

```
Copying (std::marker::Copy)
Always fast and cheap.
Always implicit.
Always identical. 
Copies are bit-for-bit duplicates of the original value.
```

> So why do Rust programmers not always use Copy? There are three main reasons:

> The Copy trait implies that there will only be negligible performance impact. This is true for numbers but not true for types that are arbitrarily large, such as String.Because Copy creates exact copies, it cannot treat references correctly. 

> Naïvely *copying a reference to T would (attempt to) create a second owner of T*. 

> That would cause problems later on because there would be multiple attempts to delete T as each reference is deleted.

## Copy Vs Vec Vs Slice
> In Rust Copy has a specific meaning of **duplicating bytes without doing any additional bookkeeping**. Vec is fundamentally incompatible with this, because it owns heap-allocated storage, which must have only one and exactly one owner. If it was allowed to be Copy, it'd be unclear which of the copies is the last one to free the storage.

> Slice &[] can be Copy, because it doesn't free the storage.

## Handle error of Copy

> But if the compiler is telling you to make something Copy, that's just a suggestion. You don't have to do this. You may have just misunderstanding of ownership, and you could avoid the need to copy by reorganizing your code, e.g.

- using shared references
- using a different data structure
- moving values to different structs or scopes.It depends on the situation.
