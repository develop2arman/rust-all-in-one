> Some general strategies for minimizing heap allocations include
* **Using arrays of uninitialized objects**. Instead of creating objects from scratch as required, create a bulk lot of those with zeroed values. When the time comes to activate one of those objects, set its values to non-zero. This can be a very dangerous strategy because you’re circumventing Rust’s lifetime checks.

* Using an allocator that is tuned for your application’s access memory profile. **Memory allocators** are often sensitive to the sizes where these perform best.
  
* **Investigate arena::Arena and arena::TypedArena**. These allow objects to be created on the fly, but alloc() and free() are only called when the arena is created and destroyed.

tags [[heap]]
