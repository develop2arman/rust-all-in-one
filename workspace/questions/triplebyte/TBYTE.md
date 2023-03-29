

[6] Which of the following snippetes sets result to the number 12?

[6.1]
```rust 
* fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()
            .filter(|&x| x % 2 ==0)
            .flat_map(|x| x * 2)
            .sum();
assert_eq!(result,12);
*}
```
[6.2]
```rust 
* fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()
            .filter(|&x| x % 2 ==0)
            .map(|x| x * 2)
            .sum();
assert_eq!(result,12);
*}
```
[6.3]
```rust 
* fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()            
            .map(|x| x * 2)
            .filter(|x| x % 2 == 0)
            .sum();
assert_eq!(result,12);
*}
```
[6.4]
```rust 
* fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()            
            .skip(1)
            .take(1)
            .map(|x| x * 2)            
            .sum();
assert_eq!(result,12);
*}
