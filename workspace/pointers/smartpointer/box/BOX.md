[[ria-smartpointer-box]]



[[rust-doc]]

[

> cons=recursive type
> We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data. By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value.Cons+Box :A List that is not infinitely sized because Cons holds a Box Cons : A List that is infinitely sized because Cons holds a Box.Cons is just a name and it is replacable with anyname.

> Boxes allow you to store data on the heap rather than the stack.

> What remains on the stack is the pointer to the heap also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

> You’ll use them most often in these situations:

> When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

> At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs.
> However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

> Box is very useful for returning traits. You also saw that we can use impl Trait to return other traits, or closures. Box can be used in a similar way. 

> You can use a Box because otherwise the compiler won't know the size of the value. dyn is a word that shows you that you are talking about a trait, not a struct or anything else.

]
