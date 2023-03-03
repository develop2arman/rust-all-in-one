
[[clp-hashmap]]

[[mr-hashmap]]

[[ria-hashmap]]

---

> The hashing algorithm used for hashing the keys of the HashMap type is based on the Robin hood open addressing scheme, >but can be replaced with a custom hasher depending on the use case and performance

## Avoid Duplication

> If we want to preserve the old values, especially if we want to update a key with no value, we can use entry() paired with or_insert()
> Default setting which is using insert you will have any duplicated key or val. 
`map.insert(key, "some value".into());`
>but when we use entry(word).or_insert(0) that will make avoid duplicated k/v.

```rust,no_run
let mut map = HashMap::<CompoundKey, String>::new();
let tmp = map.entry(word).or_insert(0);
```

---

> Snipped code
```rust,no_run
map.entry($1key_str$2).or_insert(10);,
map.entry(key).and_modify(|e| { *e += 2 });,
,
if let Entry::Occupied(o) = map.entry(key) {,
    *o.into_mut() += 2;,
},
*map.entry(key).or_insert(10)+=2;
```
