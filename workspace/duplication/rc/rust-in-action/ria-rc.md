
## RC-Immutate
> If implementing Clone would be prohibitively expensive, **Rc<T>** can be a handy alternative. This allows two places to **“share” ownership.**

> Some types overload the Clone trait. This is done to provide something similar to, yet different from, creating duplicates. For example, std::rc::Rc<T> uses Clone to create additional references when .clone() is called.

> We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case. The implementation of **Rc::clone doesn’t make a deep copy** of all the data like most types’ implementations of clone do.The call to Rc::clone only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time.

> Rc is used to track valid references. As each reference is created, an internal counter increases by one. When a reference is dropped, the count decreases by one. When the count hits zero, T is also dropped. **Wrapping T** involves a calling Rc::new().

> Rc<T> implements Clone. Every call to base.clone() increments an internal counter. Every Drop decrements that counter. When the internal counter reaches zero, the original instance is freed


## RC-Mutate
> Rc<T> does not allow mutation. To permit that, we need to wrap our wrapper.

> Rc<RefCell<T>> is a type that can be used to perform **interior mutability**

## Reference-counting semantics Vs Move semantic
> Adding more functionality (e.g., reference-counting semantics rather than move semantics) to types by wrapping these in other types typically reduces their runtime performance.
> If implementing Clone would be prohibitively expensive, Rc<T> can be a handy alternative. This allows two places to “share” ownership.
> > **Rc<T> is not thread-safe. In multithreaded code**, it’s much better to replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. #arc stands for #atomic reference counter.
