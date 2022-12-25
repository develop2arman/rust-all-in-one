
[[types-int]]

[[POINTERS]]


## Stack Vs Heap
- The stack is fast, but the heap is slow.
- That difference leads to the following axiom: **“When in doubt, prefer the stack.”** To place data onto the stack, the compiler must know the type’s size at compile time- Translated to Rust, that means, **“When in doubt, use types that implement Sized.”**
