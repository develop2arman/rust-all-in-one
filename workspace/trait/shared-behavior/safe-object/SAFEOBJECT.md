

## Dispatchable

> Dispatch is the mechanism to determine which specific version of code is actually run when it involves polymorphism. Two major forms of dispatch are static dispatch and dynamic dispatch. While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called ‘trait objects’.

>The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time. This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.

> Process of method resolution in a **polymorphic** context is called dispatch.

> **Invoking the method is called dispatching**. In mainstream languages that support polymorphism, the dispatch may happen in either of the following ways:

### Static


> When the **method to invoke is decided at compile time**, it is known as static dispatch or early binding. The method's signature is used to decide the method to call, and all of this is decided at compile time. In Rust, generics exhibit this form of dispatch because even though the generic function can accept many arguments, a specialized copy of the function is generated at compile time with that concrete type.

> The compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.

> *The monomorphized* version of the code looks like the following. The generic Option<T> is *replaced* with the specific definitions created *by the compiler*:

> Versions of a polymorphic function (or any polymorphic entity) **during compilation is called Monomorphization**.

```rust
let integer = Some(5);
let float = Some(5.0);

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```


### Dynamic

> Dynamic dispatch: In object-oriented languages, there are times when the method call can't be decided until runtime. This is because the concrete type is hidden and only interface methods are available to call on the type. In Java, this is the case when a function has an argument, which is **known as an interface**. Such a scenario can only be handled by dynamic dispatch. 

> In dynamic dispatch, **the method is determined dynamically** *by navigating through the list of implementations of the interface from* **the vtable and invoking the method**. 
> **The vtable is a list of function pointers** that point to each type's implemented method. This has a bit of **overhead** because of the extra pointer indirection in method invocation.

> In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
> When we use trait objects, Rust must use dynamic dispatch.
> The compiler doesn’t know all the types that might be used with the code that is using trait objects,so it doesn’t know which method implemented on which type to call. 

> Instead, at runtime, Rust uses the **pointers inside the trait object** to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.

```rust
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```



---

> `tags` [[trait_objects]] [[monomorphization]] [[polymorphic]]
