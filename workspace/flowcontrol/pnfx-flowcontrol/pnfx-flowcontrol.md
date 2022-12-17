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
tags #mem #size_of
