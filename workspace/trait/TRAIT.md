[[ASSOCIATE]]

[[INHERITANCE]]

[[SHARED_BEHAVIOR]]

[[SAFEOBJECT]]

---

> Traits are **private** by default.

> Traits are not usable by themselves and are meant to be implemented by types. Traits have the power to establish **relationships between distinct types**. 
> They are the **backbone** to many language features such as closures, operators, smart pointers, loops, compile-time data race checks, and much more.

---

## Marker Traits
> Traits defined in the **std::marker module** are called marker traits. **These traits don't have any method**, and simply have their declaration with their name with an empty body. Examples from the standard library include Copy, Send, and Sync. They are called marker traits because they are used to simply mark a type as belonging to a particular family for to gain some compile time guarantees. Two such examples from the standard library are the **Send and Sync traits** that are auto-implemented by the language for most types whenever appropriate, and convey which **values are safe to send and share across threads**.

---


## Glossery

  > `receiver` : for  exampele of type of Self (i.e. self) implies this 
  
