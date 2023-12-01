[[pnkfx-mpsc]]

[[ria-mpsc]]

---

> Using Message Passing to **Transfer Data Between Threads**:
> Here’s the idea in a slogan from the Go language documentation: 
> 
> **“Do not communicate by sharing memory; instead, share memory by communicating.”**

> One major tool Rust has for accomplishing message-sending concurrency is the channel,
> A channel in programming has two halves: a transmitter and a receiver. 
> A channel is said to be closed if either the transmitter or receiver half is dropped.

> We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer. In short, the way Rust’s standard library implements channels **means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.**
