[[rd-mpsc]]

---

> Message passing is a fine way of handling concurrency, but it’s not the only one. 
> the mutex is described as guarding the data it holds via the locking system.

> Mutexes have a reputation for being difficult to use because you have to remember two rules:

> You must attempt to acquire the lock before using the data.
>When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

> As you might suspect, Mutex<T> is a smart pointer. More accurately, the call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap. The MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also has a Drop implementation that releases the lock automatically 
>(e.g You might have noticed that counter is immutable but we could get a mutable reference to the value inside it;) this means Mutex<T> provides interior mutability, as the Cell family does.

> Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.


> The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread. Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.
