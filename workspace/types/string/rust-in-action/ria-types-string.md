## str
str is a high-performance, relatively feature-poor type. Once created, **str values cannot expand or shrink**. In this sense, these are similar to interacting with **a raw memory array**. Unlike a raw memory array, though, str values are guaranteed to be valid UTF-8 characters.

str is usually seen in this form: &str. A **&str (pronounced string slice) **is a small type that contains a reference to str data and a length. Attempting to assign a variable to type str will fail. The Rust compiler wants to create fixed-sized variables within a function’s stack frame. As str values can be of arbitrary length, these can only be stored as local variables by reference.

## String Vs str
A &str is a borrowed type. In practical terms, this means that **&str can be thought of as read-only data**, whereas **String is read-write.String is an owned type.**
"A String" is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity.
**String store data on heap and refs to stack**

## String-str Vs char
char—A single character encoded as **4 bytes**. The internal representation of char is equivalent to** UCS-4/UTF-32**. This differs from &str and String, which encodes single characters as UTF-8.

---

•[u8]—A #slice of raw bytes, usually found when dealing with streams of binary data.

•Vec<u8>—A #vector of raw bytes, usually created when consuming [u8] data. **String is to Vec<u8> as str is to [u8]**.

•std::ffi:: #OSString —A platform-native string. It’s behavior is close to String but without a guarantee that it’s encoded as #UTF-8 and that it won’t contain the zero byte (0x00).

•std:: #path ::Path—A string-like type that is dedicated to handling filesystem paths.

---

tags #str
