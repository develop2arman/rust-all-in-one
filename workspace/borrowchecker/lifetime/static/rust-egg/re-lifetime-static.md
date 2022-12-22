


```let s: &'static str = "I have a static lifetime.";```

The text of this string is **stored directly in the program’s binary**, which is always available. Therefore, the lifetime of all string literals is 'static.

You might see suggestions to use the 'static lifetime in error messages. 
> Most of the time, the problem results from attempting to create <u>a dangling reference or a mismatch of the available lifetimes.</u> **In such cases, the solution is fixing those problems, not specifying the 'static lifetime.**


tags #mod #const #module
