
[[types-int]]

[[POINTERS]]


## Stack

- The stack actually contains two levels of objects: #stack_frame s and data.
- The stack grants programmers access to multiple elements stored within it, rather than the top item only.
- The stack can include elements of arbitrary size, where the implication of the dinner plate(exmple theory in book) analogy is that all elements must be of the same size.

> So why is the stack called the stack?
>> Because of the usage pattern. Entries on the stack are made in a **Last In, First Out (#LIFO) manner.**The entries **in the stack are called stack frames. Stack frames are created as function calls are made**. As a program progresses, a cursor within the CPU updates to reflect the current address of the current stack frame. 
>> The #cursor is known as the [[stack_pointer]]. As functions are called within functions, the stack pointer decreases in value as the stack grows. When a function returns, the stack pointer increases.Stack frames contain a function’s state during the call. When a function is called within a function, the older function’s values are effectively frozen in time. Stack frames are also known as activation frames, and less commonly allocation records.
>> Unlike dinner plates, **every stack frame is a different size**. 
>> The stack frame contains **space for its function’s arguments**, a pointer to the original **call site, and local variables** (except the data which is allocated on the heap).

> **The stack’s primary role is to make space for local variables**. Why is the stack fast?
>> All of a function’s variables are **side by side in memory**. That speeds up access.

## Heap
> The heap is an area of program memory for types that **do not have known sizes at compile time**. 
> What does it mean to have no known size at compile time? In Rust, there are two meanings. Some types grow and shrink over time as required. 
> Obvious cases are **String and Vec<T>** . Other types are unable to tell the Rust compiler how much memory to allocate even though these don’t change size at runtime. These are known as dynamically sized types.
**Slices** ([T]) are the commonly cited example. Slices have no compile-time length. Internally, these are a pointer to some part of an array. But slices actually represent some number of elements within that array. Another example is a **trait** objects.

## Stack Vs Heap
- The stack is fast, but the heap is slow.
- That difference leads to the following axiom: **“When in doubt, prefer the stack.”** To place data onto the stack, the compiler must know the type’s size at compile time- Translated to Rust, that means, **“When in doubt, use types that implement Sized.”**
- based on Memory layout image we can say Heap in-replacable space in comprasion of stack that is LIFO

| Stack  | Heap |
| ----------- | ----------- |
| Simple | Complex |
| Safe  | Dangerous* |
| Fast**  | Slow |
| Rigid  | Flexible |
| no cost  | cost of involves dereferencing the pointer |

> "* Not in safe Rust!"
> 
> "** because a function’s local variables"


> The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating. Pushing values onto the stack is not considered allocating. Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

> Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

## What is dynamic memory allocation?
> At any given time, a running program has a fixed number of bytes with which to get its work done. 
> 
> *When the program would like more memory*, it needs to ask for more **from the OS**. 
> 
> Dynamic memory allocation is a three-step process:
- Request memory from the OS via a system call. In the UNIX family of operating systems, this system call is **alloc(). In MS Windows, the call is HeapAlloc()**.
- Make use of the allocated memory in the program.
- Release memory that isn’t needed back to the OS via **free() for UNIX systems and HeapFree() for Windows**.


## Glossery

stack = contiguous layout memory = LIFO
