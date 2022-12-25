## str
str is a high-performance, relatively feature-poor type. Once created, **str values cannot expand or shrink**. In this sense, these are similar to interacting with **a raw memory array**. Unlike a raw memory array, though, str values are guaranteed to be valid UTF-8 characters.

str is usually seen in this form: &str. A **&str (pronounced string slice) **is a small type that contains a reference to str data and a length. Attempting to assign a variable to type str will fail. The Rust compiler wants to create fixed-sized variables within a function’s stack frame. As str values can be of arbitrary length, these can only be stored as local variables by reference.

## String Vs str
A &str is a borrowed type. In practical terms, this means that **&str can be thought of as read-only data**, whereas **String is read-write.String is an owned type.**
"A String" is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity.
**String store data on heap and refs to stack**

> One (&str) is allocated on the stack, the other (String) allocates memory on the heap. That means that types cannot be trivially cast between one another. It’s possible, however, to work around this with Rust’s generics.

#As_Ref  #As_Mut #Into [[GENERIC]]

```
fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}
```

```
fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}
```

## String-str Vs char

•char—A single character encoded as **4 bytes**. The internal representation of char is equivalent to UCS-4/UTF-32. This differs from &str and String, which encodes single characters as UTF-8. Conversion does impose a penalty, but it means that char values are of fixed-width and are, therefore, easier for the [[COMPILER]]] to reason about. Characters encoded as UTF-8 can span 1 to 4 bytes.

•[u8]—**A slice of raw bytes**, usually found when dealing with streams of binary data.
It’s easy to get confused with slices [T], which do not have a compile-time length.
**Slices are dynamically** sized array-like objects.<u>The term dynamically sized means that their size is not known at compile time</u>. Yet, like arrays, these don’t expand or contract. 
The use of the word dynamic in dynamically sized is closer in meaning to dynamic typing rather than movement. 
The lack of compile-time knowledge explains the distinction in the type <u>signature between an array ([T; n ]) and a slice ([T]).</u>

Slices are important because it’s easier to implement traits for slices than arrays. Traits are how Rust programmers add methods to objects. As [T; 1], [T; 2], ..., [T; n ] are all different types, implementing traits for arrays can become unwieldy. 
> Creating a slice from an array is easy and cheap because it doesn’t need to be tied to any specific size.

Another important use for <u>slices is their ability to act as a view on arrays</u> (and other slices). The term view here is taken from database technology and means that slices can gain fast, read-only access to data without needing to copy anything around.

The problem with slices is that Rust wants to know the size of every object in your program, and **slices are defined as not having a compile-time size**. References to the rescue. 
As mentioned in the discussion about the use of the term dynamically sized, slice size is fixed in memory. 
**These are made up of two usize components (a pointer and a length). **
hat’s why you typically see slices referred to in their referenced form, &[T] (like string slices that take the notation &str).

•Vec<u8> —**A #vector of raw bytes**, usually created when consuming [u8] data. 
> String is to Vec<u8> as str is to [u8].
> Vectors (Vec<T>) are growable lists of T
Using vectors is extremely common in Rust code. These incur a small runtime penalty compared to arrays because of the extra bookkeeping that must be done to enable their size to *change over time*. But vectors almost always make up for this with their added flexibility.
Vec<T> performs best when you can provide it with a size hint via Vec:: #with_capacity(). Providing an estimate minimizes the number of times memory will need to be allocated from the OS.
Lists of things are incredibly common. The two types that you will work with most often are arrays and vectors. 
> Arrays are fixed-width and extremely lightweight.

•std::ffi:: #OSString —A platform-native string. It’s behavior is close to String but without a guarantee that it’s encoded as #UTF-8 and that it won’t contain the zero byte (0x00).

•std:: #path ::Path—A string-like type that is dedicated to handling filesystem paths.

---

tags #str #truncate
