[[EXTERN-FFI]]

[[SAFE_ABSTRACTION]]

[[SAFE_STATIC_MUT]]

[[THREAD_UNSAFE]]

---

> An unsafe block implies that the programmer takes full responsibility for any consequences

> By calling an unsafe function within an unsafe block, we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts.

```rust,compile_fail,no_run   
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```
