

All of the programmer-facing pointer types like Box<T> are built from more primitive types that live deeper within Rust, often in its core or alloc modules.


> std::mem::drop brings the function #drop () into local scope. drop() deletes objects before their scope ends.
> Types that implement Drop have a drop() method, but explicitly calling it is illegal within user code. std::mem::drop is an escape hatch from that rule.

tags [[Box]] [[Clone]]
