
[[mr-linkedlist]]

[[rd-linkedlist]]

---

> In the preceding diagram, we have two variables, var1 and var2, that reference two resources, Obj1 and Obj2. Along with that, Obj1 also has a reference to Obj2 and Obj2 has a reference to Obj1. Both Obj1 and Obj2 have reference count of 2 when var1 and var2 goes out of scope, the reference count of Obj1 and Obj2 reaches 1. They won't get freed because they still refer to each other.

![LinkedList](../../../../rust/assets/images/tree-linkedlist.JPG)

> The reference cycle can be broken using weak references. As another example, a linked list might be implemented in such a way that it maintains links via reference counting to both the next item and to the previous. A better way to do this would be to **use strong references to one direction and weak references to the other**.

> That changes a bit if we want to keep the structure this simple but **still have a double-linked list**, since then we actually have to change the existing structure.(`sample code` master-rust-linkedlist-main)

> You can **downgrade** an Rc<T> type into a Weak<T> type with the downgrade method, and similarly a Weak<T> type can be turned into Rc<T> using the **upgrade** method.

>  The downgrade method will always work. In contrast, when calling upgrade on a weak reference, the actual value might have been dropped already, in which case you get a None.(`sample code` mater-rust-linkedlist-ex-1)


> We could make append take a mutable reference to self, but that would mean that we could only append to the list if all the nodes' bindings were mutable, forcing the whole structure to be mutable. What we really want is a way to make just one small part of the whole structure mutable, and fortunately we can do that with a single RefCell.(`sample code` mater-rust-linkedlist-ex-2)

>> Whenever we're using RefCell borrows, it's a good practice to think carefully that we're using it in a safe way, since making mistakes there may lead to runtime panics. In this implementation, however, it's easy to see that we have just the single borrow, and that the closing block immediately discards it.

>> Future work: Apart from shared ownership, we can also get shared **mutability at runtime** with Rust's concept of interior mutability, which are modeled by special wrapper smart pointer types.
