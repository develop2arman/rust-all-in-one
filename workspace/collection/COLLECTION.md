[[VECTOR]]

[[SLICE]]

[[HASHMAP]]
---

> reserve more space to avoid frequent reallocations.
- String  with_capacity()
- Vec  reserve()


## Slice vs Char Vs Vec
> •char—A single character encoded as 4 bytes. The internal representation of char is equivalent to UCS-4/UTF-32. This differs from &str and String, which encodes single characters as UTF-8. Conversion does impose a penalty, but it means that char values are of fixed-width and are, therefore, easier for the compiler to reason about. Characters encoded as UTF-8 can span 1 to 4 bytes.

> •[u8]—A slice of raw bytes, usually found when dealing with streams of binary data.
> It’s easy to get confused with slices [T], which do not have a compile-time length.
> Slices are dynamically sized array-like objects. The term dynamically sized means that their size is not known at compile time. Yet, like arrays, these don’t expand or contract. The use of the word dynamic in dynamically sized is closer in meaning to dynamic typing rather than movement. The lack of compile-time knowledge explains the distinction in the type signature between an array ([T; n ]) and a slice ([T]).

> Slices are important because it’s easier to implement traits for slices than arrays. Traits are how Rust programmers add methods to objects. As [T; 1], [T; 2], ..., [T; n ] are all different types, implementing traits for arrays can become unwieldy. Creating a slice from an array is easy and cheap because it doesn’t need to be tied to any specific size.

> Another important use for slices is their ability to act as a view on arrays (and other slices). The term view here is taken from database technology and means that slices can gain fast, read-only access to data without needing to copy anything around.

> The problem with slices is that Rust wants to know the size of every object in your program, and slices are defined as not having a compile-time size. References to the rescue. As mentioned in the discussion about the use of the term dynamically sized, slice size is fixed in memory. These are made up of two usize components (a pointer and a length). That’s why you typically see slices referred to in their referenced form, &[T] (like string slices that take the notation &str).

> •Vec<u8>—A vector of raw bytes, usually created when consuming [u8] data. String is to Vec<u8> as str is to [u8].
> Vectors (Vec<T>) are growable lists of T. Using vectors is extremely common in Rust code. These incur a small runtime penalty compared to arrays because of the extra bookkeeping that must be done to enable their size to change over time. But vectors almost always make up for this with their added flexibility.
> Vec<T> performs best when you can provide it with a size hint via Vec::with_ capacity(). Providing an estimate minimizes the number of times memory will need to be allocated from the OS.
> Lists of things are incredibly common. The two types that you will work with most often are arrays and vectors. Arrays are fixed-width and extremely lightweight.

> Fully understanding the distinction between String and &str requires knowledge of arrays and vectors. Textual data is similar to these two types with added convenience methods applied over the top.

## Avoid Iterator index
> This is legal Rust. It’s also essential in cases when iterating directly over collection via for item in collection is impossible. However, it is generally discouraged. 

> The manual approach introduces two problems with this:
> Performance—Indexing values with the collection[index] syntax incurs run-time costs for bounds checking. That is, Rust checks that index currently exists within collection as valid data. Those checks are not necessary when iterating directly over collection. The compiler can use compile-time analysis to prove that illegal access is impossible.

> Safety—Periodically accessing collection over time introduces the possibility that it has changed. Using a for loop over collection directly allows Rust to guarantee that the collection remains untouched by other parts of the program.

```rust
let a = [1, 2, 3];
assert_eq!(a.iter().nth(1), Some(&2));
```
