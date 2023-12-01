[[mr_iter]]

[[pnkfx_iter]]

[[rd_iter]]

[[re_iter]]

---

Rust's for loops work can iterate over everything that implements IntoIterator. Yes, that includes Iterator itself. Apart from that, the Iterator trait has a lot of cool methods for working with the iterated values, like `filter, map, enumerate, fold, any, all, sum, min and much more`.

Did I tell you I love iterators? If your type contains more than one value of something, and it makes sense to do the same thing to all of them, consider providing an Iterator over them just in case.

Implementing Iterator is actually pretty easy – you just need to declare the Item type and write the `next(&mut self) -> Option<Self::Item>` method. This method should return Some(value) as long as you have values, then return None to stop the iteration.

Note that if you have a slice of values (or an array or vec, from which you can borrow a slice), you can get its iterator directly, so you don't even need to implement it yourself. This may not be as cool as auto-deriving, but it's nice nonetheless.
