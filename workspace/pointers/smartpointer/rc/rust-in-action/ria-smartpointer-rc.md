
## Refcell and Cell

With [[interior_mutability]], you may want to provide an argument to a method that takes immutable values, yet you need to retain mutability. If you’re willing to pay the runtime performance cost, it’s possible to fake immutability. 

> If the method requires an owned **value, wrap** the argument in *Cell<T>* . **References can also be wrapped** in *RefCell<T>*. 

## RC

> Rc is not threadsafe in spite of [[Arc]], RC is common when using the reference counted types Rc<T> and Arc<T>, which only accept immutable arguments, to also wrap those in Cell<T> or RefCell<T>. The resulting type might look like Rc<RefCell<T>>. 
> This means that you pay the **runtime cost twice (by using additional wrapper as Rc on Cell or RefCell wrapers)** but with significantly more flexibility.
> With shared ownership, some objects, such as a network connection or, perhaps, access to some OS service, are difficult to mould into the pattern of having a single place with read-write access at any given time. Code might be simplified if two parts of the program can **share access to that single resource**. Rust allows you to do this, but again, at the expense of a runtime cost.
