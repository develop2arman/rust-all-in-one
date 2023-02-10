
> Ownership is a stretched metaphor. There is no relationship to property rights. Within Rust, ownership relates to cleaning values when these are no longer needed. For example, when a function returns, the memory holding its local variables needs to be freed. Owners cannot prevent other parts of the program from accessing their values or report data theft to some overarching Rust authority.

> Ownership is a term used within the Rust community to refer to the compile-time process that checks that every use of a value is valid and that every value is destroyed cleanly.

> The ownership system can trip you up if you don’t understand what’s happening. This is particularly the case when you bring the programming style from your past experience to a new paradigm. Four general strategies can help with ownership issues:

- Use references where full ownership is not required.
- Duplicate the value.
- Refactor code to reduce the number of long-lived objects.
- Wrap your data in a type designed to assist with movement issues.
