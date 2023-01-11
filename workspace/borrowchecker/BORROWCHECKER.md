
[[LIFETIME]]

---

> It underpins the term **fearless concurrency**

Borrow checking relies on three interrelated concepts—lifetimes, ownership, and borrowing:

> Ownership: the notion of ownership is rather limited. **An owner cleans up when its values’ lifetimes end.**
For example, *when a function returns, the memory holding its local variables needs to be freed*. 
Owners cannot prevent other parts of the program from accessing their values or report data theft to some overarching Rust authority.==the term move means== something very specific within Rust. Nothing physically moves.Movement within Rust code **refers to movement of ownership**, rather than the movement of data. 
Ownership is a term used within the Rust community to <u>refer to the compile-time process that checks</u> that every use of a value is valid and that every value is destroyed cleanly.
ownership system provides a route to memory safety without needing a garbage collector. 
Four general strategies can help with ownership issues:
    - **Use references** where full ownership is not required.
    - **Duplicate** the value.
    - **Refactor** code to reduce the number of long-lived objects.
    - **Wrap** your data in a type designed to assist with movement issues.

> To borrow: ==there is no obligation to return the value to its owner==. Its meaning is used to emphasize that while values can have a **single owner**, it’s possible for many parts of the program to **share access** to those values.


## Glossery

### Ownership
[[Ownership]] has a particular meaning within Rust. An owner is able to make any changes to the data and is responsible for deleting values that it owns when it leaves scope.
