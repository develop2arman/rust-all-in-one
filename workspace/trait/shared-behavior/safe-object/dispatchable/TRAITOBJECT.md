[[dynamic dispatch]]

---

> DST : In object-oriented languages, there are times when the method call can't be decided until runtime. This is because the concrete type is hidden and only interface methods are available to call on the type. In Java, this is the case when a function has an argument, which is **known as an interface**. Such a scenario can only be handled by dynamic dispatch. 

> DST is a type without a statically known size or alignment.

> In DST, **the method is determined dynamically** *by navigating through the list of implementations of the interface from* **the vtable and invoking the method**. 
> **The vtable is a list of function pointers** that point to each type's implemented method. This has a bit of **overhead** because of the extra pointer indirection in method invocation.

> In DST cases, the compiler emits code that at runtime will figure out which method to call.
> **When we use trait objects, Rust must use DST**.
> The compiler doesn’t know all the types that might be used with the code that is using trait objects,so it doesn’t know which method implemented on which type to call. 

> Instead, at runtime, Rust uses the **pointers inside the trait object** to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch.

> **DST also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.**

```rust
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

> A trait object is implemented as **a fat pointer** and is an **unsized** type, which means that they can only be used behind references (&).

> A trait object fat pointer has **the first pointer pointing points to the actual data** associated with the object while the **second pointer to a virtual table (vtable)**

> One of the use cases for trait objects is that they allow you to operate on a collection that can have multiple types, but with an extra pointer indirection at runtime.

> A dyn Trait is an #unsized_type and can **only be created as a reference.**

```rust,no_run,compile_fail
let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
```

The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
This is opposed to DST, which is when the compiler can’t tell at compile time which method you’re calling.
In DST cases, the compiler emits code that at runtime will figure out which method to call.
When we use trait objects, Rust must use DST. The compiler doesn’t know all the types that might be used with the code that is using trait objects,
so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. DST also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.

```rust,no_run
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

> Examples of object safe methods:

```rust
fn main() {
use std::rc::Rc;
use std::sync::Arc;
use std::pin::Pin;
trait TraitMethods {
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested_pin(self: Pin<Arc<Self>>) {}
}
struct S;
impl TraitMethods for S {}
let t: Box<dyn TraitMethods> = Box::new(S);
//t.callable since object safe dispatchable
}
```
## Glossery

> Dynamic dispatch(DST)

> *Box<>, RC<>, Every trait is a dynamically sized type. Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time*. In addition, Rust implicitly adds a bound on Sized to every generic function. 

---

> `tags` [[unsized]] [[dyn]] [[fat_pointer]] [[vtable]]
