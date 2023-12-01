A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code. 
Accessing union fields is unsafe because **Rust can’t guarantee** the type of the data currently being stored in the **union instance.** 
At the time of writing this book, union types only allow Copy types as their fields. They share the same memory space with all of their fields, exactly like C unions.
