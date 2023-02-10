[[RACE]]

[[RAYON]]

[[MPSC]]

[[ATOMIC]]

[[THREAD-TIME]]

[[UNSAFE-THREAD]]

[[THREAD-SHARESTATE]]

---

> Rc<T> is not thread-safe. In multithreaded code, it’s much better to replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. [[Arc]] stands for #atomic_reference_counter.


# Thread

![thread1](../rust/assets/images/thread1.JPG)

![thread2](../rust/assets/images/thread2.JPG)

> Which parts of your code on different threads will run. This can lead to problems, such as:

- Race conditions, where threads are accessing data or resources in an inconsistent order
- Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
- Bugs that happen only in certain situations and are hard to reproduce and fix reliably

> Programming languages implement threads in a few different ways:
> APIs to create threads is sometimes called 1:1, meaning one operating system thread per one language thread.the green-threaded model is called the M:N model: there are M green threads per N operating system threads, where M and N are not necessarily the same number.
> The green-threading M:N model requires a larger language runtime to manage threads.
> As such, the Rust standard library only provides an implementation of 1:1 threading.
> Because Rust is such a low-level language, there are crates that implement M:N threading if you would rather trade > overhead for aspects such as more control over which threads run when and lower costs of context switching, for example.

> Blocking a thread means that thread is prevented from performing work or exiting.

> you can divide a calculation into independent parts, split those parts across threads, and then use a Mutex<T> to have each thread update the final result with its part.

## Thread•Strategies

**Priority Performance:**
**Stealing_Join:**
> calling join() is similar to spawning two threads.
> execute code in parallel when there are idle CPUs to handle it.
> When join is called from outside the thread pool, the calling thread will block while the closures execute in the pool. When join is called within the pool, the calling thread still actively participates in the thread pool. It will begin by executing closure A (on the current thread). While it is doing that, it will advertise closure B as being available for other threads to execute. Once closure A has completed, the current thread will try to execute closure B; if however closure B has been stolen, then it will look for other work while waiting for the thief to fully execute closure B. (This is the typical work-stealing strategy).
> Send is require because we have jump from quick func(thread a) to part func(thread b) frequently

**Atomic:**
> Atomic types provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types.
>This module defines atomic versions of a select number of primitive types, including AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU16, etc. Atomic types present operations that, when used correctly, synchronize updates between threads.

> Each method takes an Ordering which represents the strength of the memory barrier for that operation. These orderings are the same as the C++20 atomic orderings. For more information see the nomicon.
>Atomic variables are safe to share between threads (they implement Sync) but they do not themselves provide the mechanism for sharing and follow the threading model of Rust. The most common way to share an atomic variable is to put 

> it into an Arc (an atomically-reference-counted shared pointer).
> Atomic types may be stored in static variables, initialized using the constant initializers like AtomicBool::new Atomic statics are often used for lazy global initialization.

**Spin_Loop_Yeild:**
> also known as busy loop and spin loop-If you want to sleep pause a thread for short amounts of time, or if your application is sensitive to timing, use a spin loop

**Sleep:**
> A sleep is a request to the OS that the thread should be suspended until the time has passed

## Thread•Spawning
>Threads are the primary mechanism that operating systems provide for enabling concurrent execution. Modern operating systems ensure that each thread has fair access to the CPU. Understanding how to create threads (often referred to as spawning treads) and understanding their impact are fundamental skills for programmers wanting to make use of multi-core CPUs.

> threads “don’t scale.” What does that mean?
>Every thread requires its own memory, and by implication, we’ll eventually exhaust our system’s memory. Before that terminal point, though, thread creation begins to trigger slowdowns in other areas. As the number of threads to schedule increases, the OS scheduler’s work increases. When there are many threads to schedule, deciding which thread to schedule next takes more time.
> Spawning threads is not free. It demands memory and CPU time. Switching between threads also invalidates caches.

> if you’re thinking that sleeping is not a representative workload, It asks each thread to enter a spin loop. spin loop sterategy is better(performance) than sleep strategy.

> It’s also possible to use both: sleep for the bulk of the time and a spin loop towards the end.
> Second, CPU-intensive multithreading doesn’t scale well past the number of physical cores.

### Thread•Spawin•Move•Capture
> The move closure is often used alongside thread::spawn because it allows you to use data from one thread in another thread.

> we’re not using any data from the main thread in the spawned thread’s code. To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs.

> The move keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

> move closures may still implement Fn or FnMut, even though they capture variables by move. This is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them

> When the spawned thread wants to access variables that are defined in the parent’s scope, called a capture, Rust often complains that captures must be moved into the closure. To indicate that you want to move ownership, anonymous functions take a move keyword:
```rust,no_run
use std::{thread, time};
thread::spawn(move || {
    // ...
});
```

### Thread•Spawning•Join

> join is an extension of the thread metaphor. When threads are spawned, these are said to have forked from their parent thread. To join threads means to weave these back together again.

> In practice, join means wait for the other thread to finish. The join() function instructs the OS to defer scheduling the calling thread until the other thread finishes.

> .join()//means guarantee - waiting for All Threads to Finish Using join Handles. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.


### Thread•Join•WorkStealing

> Takes two closures and potentially runs them in parallel. It returns a pair of the results from those closures.

> Conceptually, calling join() is similar to spawning two threads, one executing each of the two closures. However, the implementation is quite different and incurs very low overhead. The underlying technique is called "work stealing": the Rayon runtime uses a fixed pool of worker threads and attempts to only execute code in parallel when there are idle CPUs to handle it.

> When join is called from outside the thread pool, the calling thread will block while the closures execute in the pool. When join is called within the pool, the calling thread still actively participates in the thread pool. It will begin by executing closure A (on the current thread). While it is doing that, it will advertise closure B as being available for other threads to execute. Once closure A has completed, the current thread will try to execute closure B; if 
> however closure B has been stolen, then it will look for other work while waiting for the thief to fully execute closure B. (This is the typical work-stealing strategy).


### Thread•Rec•Try

> We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel. Once a value is sent, recv will return it in a Result<T, E>. When the sending end of the channel closes, recv will return an error to signal that no more values will be coming.

> The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. Using try_recv is useful if this thread has other work to do while waiting for messages: we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again
