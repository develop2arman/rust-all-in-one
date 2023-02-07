
[[mr-ref-cell]]

---


> The RefCell type provides us with the following two borrowing methods:

- The borrow method takes a new immutable reference
- The borrow_mut method takes a new mutable reference


---
> The mutability of a binding is not fine-grained; a value is either immutable or mutable, and that includes all of its fields if it's a struct or an enum. Cell and RefCell can turn an immutable thing into something that's mutable, **allowing us to define parts of an immutable struct as mutable**.

> Whenever we're using RefCell borrows, it's a good practice to think carefully that we're using it in a safe way, since making mistakes there may lead to runtime panics. In this implementation, however, it's easy to see that we have just the single borrow, and that the closing block immediately discards it.

> Apart from shared ownership, we can also get **shared mutability at runtime** with Rust's concept of interior mutability, which are modeled by special wrapper smart pointer types.


> We need to understand the concept of interior mutability and inherited mutability:

> Inherited mutability: This is the **default** mutability you get when you take a **&mut** reference to some struct. This also implies that you can modify any of the fields of the struct.

> Interior mutability: In this kind of mutability, even if you have a **&SomeStruct reference** to some type, you can modify its fields if the fields have the type as **Cell<T> or RefCell<T>**. to ensure that no two mutable borrows are present at runtime.


## Cell

only added cost is that you have to write a bit more. The additional **runtime cost is zero**, though, and the references to the mutable things remain immutable.

The Cell<T> type is a smart pointer type that **enables mutability for values, even behind an immutable reference**. It provides this capability with very minimal overhead and has a minimal API:

> Cell::new method allows you to create new instances of the Cell type by passing it any type T.

> Get: The get method allows you to copy of the value in the cell. This method is only available if the wrapped type T is Copy.

> Set: Allows you to modify the inner value, even behind a immutable reference.

## RefCell

If you need Cell-like features for non-Copy types, there is the RefCell type. It uses a read/write pattern similar to how borrowing works, but moves the checks to runtime, which is convenient but **not zero-cost**.

## RefCell vs Cell

RefCell hands out references to the value, instead of returning things by value as is the case with the Cell type.
