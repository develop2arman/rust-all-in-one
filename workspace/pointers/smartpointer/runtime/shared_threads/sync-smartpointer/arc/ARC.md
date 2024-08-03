
> `tags` [[thread]] [[spawn]] [[smartpointer]]



## Rc VS Arc [[code-like-pro]]

> Rc is not threadsafe in spite of [[Arc]], and Unlike [Rc<T>], Arc<T> uses atomic operations for its reference counting. This means that it is thread-safe. The disadvantage is that **atomic operations are more expensive** than ordinary memory accesses. 
> 
>If you are not sharing reference-counted allocations between threads, consider using **[Rc<T>] for lower overhead**.
>
>[Arc<T>] is a safe default, because the compiler will catch any attempt to send an [Rc<T>] between threads. However, a library might choose Arc<T> in order to give library consumers more flexibility.

> > **Rc<T> is not thread-safe. In multithreaded code**, it’s much better to replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. [[Arc]] stands for #atomic reference counter.