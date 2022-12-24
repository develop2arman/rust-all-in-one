
[[ria-pointer]]

[[SMARTPOINTER]]


> What are the differences between references, pointers, and memory addresses?

- **A memory address**, often shortened to address, is a number that happens to refer to a single byte in memory. Memory addresses are abstractions *provided by assembly languages*.
>> Used **for types** where it’s important to make their **unsafe** nature explicit.

- **A pointer**, sometimes expanded to raw pointer, is a memory address that points to a value of some type. Pointers are abstractions provided by *higher-level languages*.
>> Refer to something more **primitive**. This also includes the implication that **we are responsible** for maintaining safety. (There is an implied connotation of being **unsafe**.)

- **A reference** *is a pointer*, or in the case of *dynamically sized types*, a pointer and an integer with extra guarantees. References are abstractions *provided by Rust*.
>> References—Signal that the **Rust compiler** will provide its **safety guarantees**.
  - > References always refer to valid data.
  - > References are correctly aligned to multiples of usize
  - > Rust ensures that *a length* is kept alongside the *internal pointer*. That way Rust can ensure that the program never overruns the type’s space in memory.

---



![Memory-Layout-1](../rust/assets/images/mem-layout1.JPG)
![Memory-Layout-2](../rust/assets/images/mem-layout2.JPG)

![Dynamic Memory](../rust/assets/images/dynamic-mem.JPG)
