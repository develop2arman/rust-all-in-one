

> A trait object is implemented as **a fat pointer** and is an **unsized** type, which means that they can only be used behind references (&).

> A trait object fat pointer has **the first pointer pointing points to the actual data** associated with the object while the **second pointer to a virtual table (vtable)**

> One of the use cases for trait objects is that they allow you to operate on a collection that can have multiple types, but with an extra pointer indirection at runtime.

> A dyn Trait is an unsized type and can only be created as a reference.

```rust,no_run,compile_fail
let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
```

The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.
In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that is using trait objects,
so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.

```rust,no_run
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```
## Glossery

> `DSTs`: unsized types - str(but not &str-So although a &T is a single value that stores the memory address of where the T is located, a &str is two values: the address of the str and its length. As such, we can know the size of a &str value at compile time),
> *Box<>, RC<>, Every trait is a dynamically sized type.Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time*. In addition, Rust implicitly adds a bound on Sized to every generic function. 

---

> `tags` [[unsized]] [[dyn]] [[fat_pointer]] [[vtable]]
