[[mr-builtin-trait]]

---

## Marker/Fundamental Traits
> Rust identifies these types with a special trait called Copy. **Copy is known as a marker trait**.
> Traits defined in the **std::marker module** are called marker traits. **These traits don't have any method**, and simply have their declaration with their name with an empty body. Examples from the standard library include Copy, Send, and Sync. They are called marker traits because they are used to simply mark a type as belonging to a particular family for to gain some compile time guarantees. Two such examples from the standard library are the **Send and Sync traits** that are auto-implemented by the language for most types whenever appropriate, and convey which **values are safe to send and share across threads**.

## Send & Sync
Those two traits testify that the types transfer trouble-free 'tween threads.
You will never need to implement them – in fact Rust will do it for you by default unless you explicitly opt out (or your type contains a non-threadsafe part). You can opt out by saying:

```rust,no_run,compile_fail
impl !Send for MyType {} // this type cannot be sent to other threads
impl !Sync for MyType {} // nor can it be used by two of them
```
Send says that you can move your type between thread barriers, while Sync allows sharing a value between threads.

```rust,no_run,compile_fail
impl<'a, T> Send for &'a T where T: Sync + ?Sized
```

This means if something can be Synced, you can Send a reference to it between threads.
For a more thorough treatment, see Manish Goregaokar's [How Rust Achieves Thread Safety](http://manishearth.github.io/blog/2015/05/30/how-rust-achieves-thread-safety) or the [Sync docs](http://doc.rust-lang.org/std/marker/trait.Sync.html).


## Default

Default is a trait to declare a default value for your type. It can be auto-derived, but only for structs whose members all have a Default implementations.

It is implemented for a great many types in the standard libraries, and also used in a surprising number of places. So if your type has a value that can be construed as being "default", it is a good idea to implement this trait.

A great thing with structs that have a Default implementation, is you can instantiate them with only the non-default values like:

```rust,no_run,compile_fail
let x = Foo { bar: baz, ..Default::default() }
```

## Hash

Hashing is the process of reducing a bag of data into a single value that still distinguishes different data items while returning the same value for equal items without requiring as much bits as the processed data.

In Rust, the Hash trait denotes values to which this process can be applied. Note that this trait does not relate any information about the hash algorithm used (this is encapsulated within the Hasher trait), it basically just orders the bits to be hashed.

Aside: This is also the reason why HashMap does not implement Hash itself, because two equal hash maps could still store their contents in different order, resulting in different hashes, which would break the hashing contract. Even if the items were ordered (see Ord above), hashing them would require sorting, which would be too expensive to be useful. One could also xor the entry hash values, but that would require re-using the Hasher, which would at least require a Clone bound, which the interface lacks. In any event, use a BTreeMap as key for your maps if you must have maps as keys to a hashmap.

Unless you have some very specific constraints regarding equality, you can safely auto-derive Hash. Should you choose to implement it manually, be careful not to break its contract, lest your programs fail in surprising and hard to debug ways.

