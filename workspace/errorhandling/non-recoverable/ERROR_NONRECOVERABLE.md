

[[mr-non-recoverable-errorhandling]]

---

When code that's in the execution phase encounters a bug, or one of its variants is violated, it has the potential to corrupt the program state in unexpected ways if it's ignored. These situations are deemed non-recoverable because of their inconsistent program state, which may **lead to faulty outputs or unexpected behavior later**. 
> This means that a fail-stop approach is the best way to recover from them so as to not harm other parts or systems indirectly.
For these kinds of cases, Rust provides us with a mechanism called panic, which aborts the thread on which it is invoked and does not affect any other threads. If the main thread is the one facing the panic, then the program aborts with a non-zero exit code of 101. If it's a child thread, the panic does not propagate to the parent thread and halts at the thread boundary. 
> A panic in one thread does not affect the other threads and is isolated, except in cases where they corrupt a mutex lock on some shared data; it is implemented as a macro by the same panic! mechanism.

> When panic! is called, **the panicking thread starts unwinding the function call stack**, starting from the place at which it was invoked, all the way until the entry point in the thread. It also generates a stack trace or a backtrace for all functions that are invoked in this process, just like exceptions. But in this case, it does not have to look for any exception handlers, as they don't exist in Rust. 

> **Unwinding** is the process of moving up the function call chain while **cleaning up or freeing resource**, from each function call stack.

>  These resources can be stack allocated or heap allocated. Stack allocated resources automatically get released once the function ends. For variables pointing to heap allocated resources, Rust calls the drop method on them, which frees up the memory used by the resource. 

> **This cleanup is necessary to avoid memory leaks**. Apart from code calling panic explicitly, Result/Option error types also call panic if any code does.

In the case of single-threaded code having panics on the main thread, unwinding doesn't provide much of a benefit, as the operating system reclaims all the memory after the process aborts. 

> Fortunately, there are options to **turn off unwinding in panic**, which may be required on platforms such as **embedded systems**, where we have a single main thread doing all the work and where unwinding is an expensive operation that isn't of much use.

To figure out the sequence of calls that led to the panic, we can view the backtrace from the thread by running any panicking program and setting the **RUST_BACKTRACE=1** environment variable from our command-line shell.
