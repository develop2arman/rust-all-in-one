[[BOX]]

[[RC]]

---

## Comparistions of Smart pointers 

> Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.

> Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.

> Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

> RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.

> Similar to Rc<T>, RefCell<T> is only for use in [[single_threaded]] scenarios.

> Neither Cell<T> nor RefCell<T> are thread safe (they do not implement #Sync )

> Standard library has other types that provide interior mutability:

>> Such as Cell<T>, which is similar except that instead of giving references to the inner value, the **value is copied** in and out of the Cell<T>.

>> There’s also Mutex<T>, which offers interior mutability that’s safe to use **across threads** [[multi_tread]] scenarios.
