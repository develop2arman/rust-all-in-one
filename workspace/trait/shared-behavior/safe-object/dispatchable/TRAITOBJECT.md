

> A trait object is implemented as **a fat pointer** and is an **unsized** type, which means that they can only be used behind references (&).

> A trait object fat pointer has **the first pointer pointing points to the actual data** associated with the object while the **second pointer to a virtual table (vtable)**

> One of the use cases for trait objects is that they allow you to operate on a collection that can have multiple types, but with an extra pointer indirection at runtime.

> A dyn Trait is an unsized type and can only be created as a reference.
```
let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
```

> `tags` [[unsized]] [[dyn]] [[fat_pointer]] [[vtable]]
