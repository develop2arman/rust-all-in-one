[[LOOP]]

[[WHILE]]]

[[MATCH]]

[[pnfx-flowcontrol]]

---

## Iter Vs For performance

`test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)`
`test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)`
`The iterator version was slightly faster! We won’t explain the benchmark code here, because the point is not to prove that the two versions are equivalent but to get a general sense of how these two implementations compare performance-wise.`



```
use std::mem;
println!(" size_of::<&[i32; 9]>() == {:2} bytes", mem::size_of::<&[i32; 9]>());
println!("      size_of::<&[i32]>() == {:2} bytes", mem::size_of::<&[i32]>());
```
> Result:
```
size_of::<&[i32; 9]>() ==  8 bytes
size_of::<&[i32]>() == 16 bytes
```

> `tags` [[benchmark]]
