
•Rust doesn't support f8,f16.
 
• f32 and f64 types only implement the std::cmp::PartialEq trait, whereas other numeric types also implement std::cmp::Eq.

• To prevent these hazards, here are two guidelines to follow:

> • Avoid testing floating-point numbers for equality.

> • Be wary when results may be mathematically undefined.

• assert! crashes the program unless its argument evaluates to true.

=== "Example 1"

    ```rust,compile_fail,ignore
    fn main() {
    assert!(0.1 + 0.2 == 0.3);
    }
    ```

=== "Example 2"

    ```rust,compile_fail,ignore
    fn main() {
     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
 
     println!("abc (f32)");
     println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
     println!("         0.3: {:x}", (abc.2).to_bits());
     println!();
 
     println!("xyz (f64)");
     println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
     println!("         0.3: {:x}", (xyz.2).to_bits());
     println!();
 
     assert!(abc.0 + abc.1 == abc.2);
     assert!(xyz.0 + xyz.1 == xyz.2);
    }
    ```

=== "Result"

```py hl_lines="6 7"
    abc (f32)
    0.1 + 0.2: 3e99999a
         0.3: 3e99999a
 
    xyz (f64)
   0.1 + 0.2: 3fd3333333333334
         0.3: 3fd3333333333333
 
    thread 'main' panicked at 'assertion failed: xyz.0 + xyz.1 == xyz.2',
    ➥ch2-add-floats.rs.rs:14:5
    note: run with `RUST_BACKTRACE=1` environment variable to display
    ➥a backtrace
```


tags #from_bits #hex #to_bits
