
[[rust-doc]]

[

> To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting. The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

> Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

> We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

> Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever one cloned Rc is dropped out of the scope

>> **The definition of Cons to hold references instead, but then we would have to specify lifetime parameters.**

> By specifying [[lifetime]] parameters, we would be specifying that every element in the list will live at least as long as the entire list.

> multiple [[mutable_borrow]] s to the same place can cause [[data_race]] s and inconsistencies.

> But being able to mutate data is very useful! In the next section, mutability pattern and the RefCell<T> type that you can use in conjunction with an Rc<T> to work with this [[interior_immutability]] restriction

> Using Rc<T> came with the **risk of creating reference cycles, where two Rc<T>** values refer to each other, causing [[memory_leaks]].

]

## Refcell and Cell

With [[interior_mutability]], you may want to provide an argument to a method that takes immutable values, yet you need to retain mutability. If you’re willing to pay the runtime performance cost, it’s possible to fake immutability. 

> If the method requires an owned **value, wrap** the argument in *Cell<T>* . **References can also be wrapped** in *RefCell<T>*. 

## RC

> Rc is not threadsafe in spite of [[Arc]], RC is common when using the reference counted types Rc<T> and Arc<T>, which only accept immutable arguments, to also wrap those in Cell<T> or RefCell<T>. The resulting type might look like Rc<RefCell<T>>. 
> This means that you pay the **runtime cost twice (by using additional wrapper as Rc on Cell or RefCell wrapers)** but with significantly more flexibility.
> With shared ownership, some objects, such as a network connection or, perhaps, access to some OS service, are difficult to mould into the pattern of having a single place with read-write access at any given time. Code might be simplified if two parts of the program can **share access to that single resource**. Rust allows you to do this, but again, at the expense of a runtime cost.



## Rc VS Arc [[rust-doc]]

> Unlike [Rc<T>], Arc<T> uses atomic operations for its reference counting. This means that it is thread-safe. The disadvantage is that **atomic operations are more expensive** than ordinary memory accesses. 
> 
>If you are not sharing reference-counted allocations between threads, consider using **[Rc<T>] for lower overhead**.
>
>[Rc<T>] is a safe default, because the compiler will catch any attempt to send an [Rc<T>] between threads. However, a library might choose Arc<T> in order to give library consumers more flexibility.
