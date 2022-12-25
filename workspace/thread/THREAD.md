[[RACE]]


> Rc<T> is not thread-safe. In multithreaded code, it’s much better to replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. [[Arc]] stands for #atomic_reference_counter.
