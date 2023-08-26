
[[ria-smartpointer-rc]]

[[rd-smartpointer-rc]]

[[LINKEDLIST]]

---

> [[SemiAutomatic]]


> To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting. The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

> Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

> We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

> Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever one cloned Rc is dropped out of the scope

>> **The definition of Cons to hold references instead, but then we would have to specify lifetime parameters.**

> By specifying [[lifetime]] parameters, we would be specifying that every element in the list will live at least as long as the entire list.

> multiple [[mutable_borrow]] s to the same place can cause [[data_race]] s and inconsistencies.

> But being able to mutate data is very useful! In the next section, mutability pattern and the RefCell<T> type that you can use in conjunction with an Rc<T> to work with this [[interior_immutability]] restriction

> Using Rc<T> came with the **risk of creating reference cycles, where two Rc<T>** values refer to each other, causing [[memory_leaks]].


# Interior mutability is a design pattern

> That allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. 

> RefCell<T> type that follows the interior mutability pattern.

> Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So, what makes RefCell<T> different from a type like Box<T>? Recall the borrowing rules you learned in Chapter 4:

> Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

> At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.References must always be valid.

> RefCell<T> uses Rust’s lifetimes to implement ‘dynamic borrowing’, a process whereby one can claim temporary, exclusive, mutable access to the inner value.
> Because RefCell<T> borrows are dynamic it is possible to attempt to borrow a value that is already mutably borrowed; when this happens it results in thread panic.

## Refcell and Cell

With [[interior_mutability]], you may want to provide an argument to a method that takes immutable values, yet you need to retain mutability. If you’re willing to pay the runtime performance cost, it’s possible to fake immutability. 

> If the method requires an owned **value, wrap** the argument in *Cell<T>* . **References can also be wrapped** in *RefCell<T>*. 

## RC

> Rc is not threadsafe in spite of [[Arc]], RC is common when using the reference counted types Rc<T> and Arc<T>, which only accept immutable arguments, to also wrap those in Cell<T> or RefCell<T>. The resulting type might look like Rc<RefCell<T>>. 

> This means that you pay the **runtime cost twice (by using additional wrapper as Rc on Cell or RefCell wrapers)** but with significantly more flexibility.

> With shared ownership, some objects, such as a network connection or, perhaps, access to some OS service, are difficult to mould into the pattern of having a single place with read-write access at any given time. Code might be simplified if two parts of the program can **share access to that single resource**. Rust allows you to do this, but again, at the expense of a runtime cost.

> Rc internally keeps two kinds of references: strong **(Rc<T>)** and weak **(Weak<T>)**. Both keep a count of how many references of each type have been handed out.

---

> To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting. The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

> Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

> We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

> Note that Rc<T> is only for use in single-threaded scenarios.

> Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever one cloned Rc is dropped out of the scope

> the definition of Cons to hold references instead, but then we would have to specify lifetime parameters. 
By specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire list.

> multiple mutable borrows to the same place can cause data races and inconsistencies.
But being able to mutate data is very useful! In the next section, mutability pattern and the RefCell<T> type that you can use in conjunction with an Rc<T> to work with this Interior immutability restriction

> using Rc<T> came with the risk of creating reference cycles, where two Rc<T> values refer to each other, causing memory leaks. 

## Rc VS Arc [[code-like-pro]]

> Unlike [Rc<T>], Arc<T> uses atomic operations for its reference counting. This means that it is thread-safe. The disadvantage is that **atomic operations are more expensive** than ordinary memory accesses. 
> 
>If you are not sharing reference-counted allocations between threads, consider using **[Rc<T>] for lower overhead**.
>
>[Rc<T>] is a safe default, because the compiler will catch any attempt to send an [Rc<T>] between threads. However, a library might choose Arc<T> in order to give library consumers more flexibility.



## RC-Immutate
> If implementing Clone would be prohibitively expensive, **Rc<T>** can be a handy alternative. This allows two places to **“share” ownership.**

> Some types overload the [[Clone]] trait. This is done to provide something similar to, yet different from, creating duplicates. For example, std::rc::Rc<T> uses Clone to create additional references when .clone() is called.

> We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case. The implementation of **Rc::clone doesn’t make a deep copy** of all the data like most types’ implementations of clone do.The call to Rc::clone only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time.

> Rc is used to track valid references. As each reference is created, an internal counter increases by one. When a reference is dropped, the count decreases by one. When the count hits zero, T is also dropped. **Wrapping T** involves a calling Rc::new().

> Rc<T> implements Clone. Every call to base.clone() increments an internal counter. Every Drop decrements that counter. When the internal counter reaches zero, the original instance is freed


## RC Mutate

> Rc<T> does not allow mutation. To permit that, we need to wrap our wrapper.

> Rc<RefCell<T>> is a type that can be used to perform **interior mutability**

## Reference counting semantics Vs Move semantic

> Adding more functionality (e.g., reference-counting semantics rather than move semantics) to types by wrapping these in other types typically reduces their runtime performance.
> If implementing Clone would be prohibitively expensive, Rc<T> can be a handy alternative. This allows two places to “share” ownership.
> > **Rc<T> is not thread-safe. In multithreaded code**, it’s much better to replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. [[Arc]] stands for #atomic reference counter.



---

> `tags` [[pattern_design_interior]]
