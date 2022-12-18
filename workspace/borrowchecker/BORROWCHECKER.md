
[[LIFETIME]]


### Let vs Const

> If variables defined with #let are immutable, then why does Rust include a #const keyword?
 
The short answer is that data behind let can change. Rust allows types to have an apparently contradictory property of interior mutability.
At the level of the [[COMPILER]]], let relates more to #alias ing than immutability.
Aliasing in compiler terminology refers to having multiple references to the same location in memory at the same time. 
**Read-only references** (borrows) to variables declared with **let** can alias the same data. 
**Read-write references** (mutable borrows) are guaranteed to **never alias data.**
Some types such as std:sync::Arc and std:#rc::Rc present an immutable façade, yet change their internal state over time. In the case of those two types, these increment a #reference_count as references to those are made and decrement that count when those references expire.
