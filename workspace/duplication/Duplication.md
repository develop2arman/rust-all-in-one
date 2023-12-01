

[[ria-copy-clone]]

[[ria-rc]]

---

> One alternative to refactoring is to simply copy values. Doing this often is typically frowned upon, however, but it can be useful in a pinch. Primitive types like integers are a good example of that. Primitive types are cheap for a CPU to duplicate—so cheap, in fact, that Rust always copies these if it would otherwise worry about ownership being moved.

> Types can opt into two modes of duplication: cloning and copying.
