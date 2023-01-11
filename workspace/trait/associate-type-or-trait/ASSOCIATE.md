
## Trait Methods

> We can have two kinds of methods within a trait: Associated methods, Instance methods

### Associated methods
> These are methods that are available directly on the type implementing the trait and do not need an instance of the type to invoke them. There are also known **as static methods** in mainstream languages, **for example, the from_str method from the FromStr trait** in the standard library. *It is implemented* for a String and thus allows you to create a String from a &str by calling String::from_str("foo").

### Instance methods
> These are **methods that have their first parameter as self**. These are only available on instances of the type that are implementing the trait.  self points to the instance of the type implementing the trait. It can be of three types: self  methods, which consume the instance when called; &self methods, which only have read access to the instance its members (if any); and &mut self methods, which have mutable access to its members and can modify them or even replace them with another instance. **For example, the as_ref method from the AsRef trait** in the standard library is an instance method that takes &self, and is meant to be implemented by types that can be converted to a reference or a pointer.


## Associated type traits
> This removes the redundant specification of types, as is the case with generic traits. One of the finest examples of associated type traits is the Iterator trait.
- [x] The advantage of them is that, in the implementation, they allow us to declare the assciated type once and use **Self::Out as the return type** or parameter type in any of the trait methods or functions.

```rust,compile_fail,no_run,ignore
trait Foo {
    type Out;
    fn get_value(self) -> Self::Out;
}
```

> `tags` [[associate_method]] [[instance_method]] [[associate_trait]]
