[[BOX]]

[[RC]]

[[REF_CELL]]

[[OPS]]

[[COW]]

[[rd-customized-smartpointer-box]]

---

## What are Smart Pointers?

> They are called smart because they also have extra metadata and code associated with them that gets executed when they are created or destroyed. Being able to **automatically free the underlying resource** when a smart pointer goes out of scope is one of the major reasons to use smart pointers.
> Much of the smartness in smart pointers comes from two traits, called the **Drop trait and the Deref trait**.


### Drop Mechanism
> When we instantiate any Drop implementing value (any **heap** allocated type), the Rust compiler inserts drop method calls after every end of scope, after compilation. So, we don't need to manually call drop on these instances. This kind of automatic reclamation based on scope is inspired by the **RAII** principle of C++.

### Drerf Mechanism
> smart pointer types often implement the Deref trait, which allows us to use the * dereferencing operator with these types. While **Deref gives you read-only access**, there is also **DerefMut, which can give you a mutable reference** to the underlying type. Deref has the following type signature:

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

> Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref coercion works only on types that implement the Deref trait. 

> **Deref coercion converts such a type into a reference to another type**. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str. 

> The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

> Similar to how you use the **Deref trait to override the * operator on immutable references**, you can use the DerefMut trait to override the * operator on mutable references.

> Rust does deref coercion when it finds types and trait implementations in three cases:


- From &T to &U when T: Deref<Target=U>
- From &mut T to &mut U when T: DerefMut<Target=U>
- From **&mut T to &U** when T: **Deref**<Target=U>
- **The third case** is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible

> the Drop trait is almost always used when implementing a smart pointer. For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.

> *Note that we didn’t need to call the+ drop method explicitly.*

> State is a trait, we call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value. 
> Because state is an `Option<Box<dyn State>>`

```rust,no_run,compile_fail  
  impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
```
> At this point, when we call content on the `&Box<dyn State>`, deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait.

> If defines a single method called Deref that takes self by reference and returns a immutable reference to the underlying type. This combined with the deref coercion feature of Rust, reduces a lot of code that you have to write. Deref coercion is when a type automatically gets converted from one type of reference to some other reference.

## Comparistions of Smart pointers 

![Ownership](../../rust/assets/images/Ownership.jpg)


---

### Pointer Definations & Comparision

> Rc<T> enables multiple owners of the same data; whenever somebody takes a new reference, and decrements it when someone releases a reference. When the counter hits zero, the value is dropped.Rc<T> does not allow mutation. To permit that, we need to wrap our wrapper. Rc<RefCell<T>> is a type that can be used to perform interior mutability . An object that has interior mutability presents an immutable façade while internal values are being modified.

> Rc<T> is not thread-safe. In multithreaded code, it’s much better to replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. Arc stands for atomic reference counter.

> Box<T> The ownership semantics with Box type depends on the wrapped type. If the underlying type is Copy, the Box instance becomes copy, otherwise it **moves by default**.allows immutable or mutable borrows checked at compile time; **Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.**

> Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

> Arc<T>: This is for atomic reference counting. This is like the previous type, but with atomicity to guarantee multithread safety.

> Cell<T>: This gives us internal mutability for types that implement the Copy trait. In other words, we gain the possibility to get multiple mutable references to something.

> RefCell<T>: This gives us internal mutability for types, without requiring the Copy trait. It uses **runtime locking for safety**. Lets us have many immutable borrows or one mutable borrow at any point in time.Box<T> and RefCell<T> have single owners. For types that implement Copy, the get method retrieves the current interior value. For types that implement Default, the take method replaces the current interior value with Default::default() and returns the replaced value.For all types, the replace method replaces the current interior value and returns the replaced value and the **into_inner** method consumes the Cell<T> and returns the interior value. Additionally, the set method replaces the interior value, dropping the replaced value.

![Smart-Pointer-1](../../rust/assets/images/smart-pointer-1.JPG)

![Smart-Pointer-2](../../rust/assets/images/smart-pointer-2.JPG)

![Smart-Pointer-3](../../rust/assets/images/smart-pointer-3.JPG)



### Comparison of inherited and interior/mutability

> We say that Cell<T> and RefCell<T> provide ‘interior mutability’, in contrast with typical Rust types that exhibit **‘inherited mutability’ the pattern uses unsafe** code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.

> Similar to Rc<T>, RefCell<T> is only for use in [[single_threaded]] scenarios.The Cell and RefCell types are not thread safe. This simply means that **Rust won't allow you to share these types in multiple threads**.

> **Neither Cell<T> nor RefCell<T> are thread safe (they do not implement #Sync)**
> Standard library has other types that provide interior mutability:

>> Such as Cell<T>, which is similar except that instead of giving references to the inner value, the **value is copied** in and out of the Cell<T>.

>> There’s also Mutex<T>, which offers interior mutability that’s **safe** to use **across threads** [[multi_tread]] scenarios.
