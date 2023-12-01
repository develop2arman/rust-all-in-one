

[[mr-non-recoverable-errorhandling]]

---

When code that's in the execution phase encounters a bug, or one of its variants is violated, it has the potential to corrupt the program state in unexpected ways if it's ignored. These situations are deemed non-recoverable because of their inconsistent program state, which may **lead to faulty outputs or unexpected behavior later**. 
> This means that a fail-stop approach is the best way to recover from them so as to not harm other parts or systems indirectly.
For these kinds of cases, Rust provides us with a mechanism called panic, which aborts the thread on which it is **invoked and does not affect any other threads**. If the main thread is the one facing the panic, then the program aborts with a non-zero exit code of 101. If it's a child thread, the panic does not propagate to the parent thread and halts at the thread boundary. 
> A panic in one thread does not affect the other threads and is isolated, except in cases where they corrupt a mutex lock on some shared data; it is implemented as a macro by the same panic! mechanism.

> **Unwinding** is the process of moving up the function call chain while **cleaning up or freeing resource**, from each function call stack.

> When panic! is called, **the panicking thread starts unwinding the function call stack**, starting from the place at which it was invoked, all the way until the entry point in the thread. It also generates a stack trace or a backtrace for all functions that are invoked in this process, just like exceptions. But in this case, it does not have to look for any exception handlers, as they don't exist in Rust. 

>  These resources can be stack allocated or heap allocated. Stack allocated resources automatically get released once the function ends. For variables pointing to heap allocated resources, Rust calls the drop method on them, which frees up the memory used by the resource. 

> **This cleanup is necessary to avoid memory leaks**. Apart from code calling panic explicitly, Result/Option error types also call panic if any code does.

In the case of **single-threaded** code having panics on the main thread, **unwinding doesn't provide much of a benefit**, as the operating system reclaims all the memory after the process aborts. 

> Fortunately, there are options to **turn off unwinding in panic**, which may be required on platforms such as **embedded systems**, where we have a single main thread doing all the work and where unwinding is an expensive operation that isn't of much use.

To figure out the sequence of calls that led to the panic, we can view the backtrace from the thread by running any panicking program and setting the **RUST_BACKTRACE=1** environment variable from our command-line shell.


> you can use the std::panic::catch_unwind function. Even though it's recommended to handle errors via the Option/Result mechanism.

> `catch_unwind` doesn't prevent the panic – it only allows you to **customize the unwind behavior** associated with panic. panic with **catch_unwind is not recommended as a general error handling** method for Rust programs.

```rust
fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> 
```

As you can see, the return value of **catch_unwind** has an additional constraint, UnwindSafe. This means that the variables in the closure must be exception-safe, which most types are, but notable exceptions are mutable references (&mut T). A value is exception safe if exception-throwing code cannot lead to the value being left in an inconsistent state. This means that the **code inside the closure must not panic!() itself**.

it just **stops the unwinding associated with the panicking thread**. Note again that catch_unwind is not the recommended method for error management in Rust. It is not guaranteed to catch all panics, such as panics that abort the program. 
> > Catching panic unwinding is **necessary** in situations where Rust code is communicating with other languages such as C, **where unwinding to C code **is an undefined behavior.

> The program can then resume the unwind by using the **resume_unwind** function from the same panic module.


For rare cases where the default unwinding behavior of panic can get too expensive, such as when writing programs for microcontrollers, *there's a compiler flag that can be configured to turn all panics into aborts*. To do that, your project's Cargo.toml needs to have the following attribute under the profile.release section:

```
[profile.release]
panic = "abort"
```

### User-Friendly Panics

verbose, cryptic panic messages with human-readable messages. It also *writes the backtrace to a file* to allow it to be reported to the tool author by users.
