[[ria-smartpointer-box]]


---


> The Box type can be used in the following situations:

> It can be used to create **recursive type** definitions. 

> When you need to **store types as trait objects**.

> When you need to **store functions in a collection**.

> cons=recursive type
> 
> We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data. By using a box, **we’ve broken the infinite**, recursive chain, so the **compiler can figure out the size it** needs to *store a List value*.
> Cons+Box :A List that is not infinitely sized because Cons holds a Box Cons : A List that is infinitely sized because Cons is just a name and it is replacable with anyname.
>
>> Boxes allow you to store data on the heap rather than the stack.

> What remains on the stack is the pointer to the heap also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.
> >>The Box<T> type is a [[smart_pointer]] because it implements the #Deref trait, 
> 
> **which allows Box<T> values to be treated like references**. When a Box<T> value goes out of scope, the [[heap]] data that the box is pointing to is cleaned up as well because of the [[Drop]] [[trait]] implementation.

> You’ll use them most often in these situations:

> **When you have a type whose size can’t be known at compile time** and you want to use a value of that type in a context that requires an exact size **When you have a large amount of data** and you want to transfer ownership but ensure the data won’t be copied when you do so When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

> At [[compile_time]] , Rust needs to know how much space a type takes up. One type **whose size can’t be known at compile time is a recursive type**, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs.
> 
> However, **boxes have a known size**, so by inserting a box in a recursive type definition, you can have recursive types.

> **Box is very useful for returning traits**. You also saw that we can use impl Trait to return other traits, or closures. Box can be used in a similar way. 

> You can use a **Box because otherwise the compiler won't know the size of the value**. 
> 
>> [[dyn]] is a word that shows you that you are talking about a trait, not a struct or anything else.


## Recursive

> At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

> Let’s explore the cons list, which is a data type common in functional programming languages, as an example of a recursive type. The cons list type we’ll define is straightforward except for the recursion; therefore, the concepts in the example we’ll work with will be useful any time you get into more complex situations involving recursive types.
> A cons list is a data structure.

> cons function (short for “construct function”) constructs a new pair from its two arguments, which usually are a single value and another pair. These pairs containing pairs form a list.
> cons x onto y” informally means to construct a new container instance by putting the element x at the start of this new container, followed by the container y.
>
> Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item. A cons list is produced by recursively calling the cons function. The canonical name to denote the base case of the recursion is Nil.
> The canonical name to denote the base case of the recursion is Nil. Note that this is not the same as the “null” or “nil” concept.

> Although functional programming languages use cons lists frequently, the cons list isn’t a commonly used data structure in Rust. Most of the time when you have a list of items in Rust, Vec<T> is a better choice to use. Other, more complex recursive data types are useful in various situations, but by starting with the cons list, we can explore how boxes let us define a recursive data type without much distraction.
> Nil variant stores no values, so it needs less space than the Cons variant.

## Drop Box
> All of the programmer-facing pointer types like Box<T> are built from more primitive types that live deeper within Rust, often in its core or alloc modules.

> std::mem::drop brings the function #drop () into local scope. drop() deletes objects before their scope ends.
> Types that implement Drop have a drop() method, but explicitly calling it is illegal within user code. std::mem::drop is an escape hatch from that rule.


---

> `tags` [[recursive]]
