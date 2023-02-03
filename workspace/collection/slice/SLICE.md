
[[clp-arithmetic]]

[[mr_iter]]

[[rd-slice]]

---

> Slices are a **generic way** to get **a view into a collection** type. Most use cases are to get a read only access to a certain range of items in a collection type. A slice is basically **a pointer or a reference** that points to a continuous range in an existing collection type that's owned by some other variable.
> 
>  Under the hood, slices are #fat_pointer s to existing data somewhere in the stack or the heap. By fat pointer, it means that they also have information on *how many elements they are pointing to*, along with the pointer to the data.
>
> we can't have slices as bare values – only behind a pointer. This is because **slices are unsized types**.

> Note: The &str type also comes under the category of a slice type (a [u8]). The only distinction from other byte slices is that they are guaranteed to be UTF-8. Slices can also be taken on Vecs or Strings.

> Slicing can be **shared and immutable**.

---

> `tags` unsize
