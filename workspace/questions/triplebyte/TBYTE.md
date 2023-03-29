

[6] Which of the following snippetes sets result to the number 12?

[6.1]
```rust 
#  fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()
            .filter(|&x| x % 2 ==0)
            .flat_map(|x| x *  2)
            .sum();
assert_eq!(result,12);
# }
```
**[6.2]**
```rust 
#  fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()
            .filter(|&x| x % 2 ==0)
            .map(|x| x *  2)
            .sum();
assert_eq!(result,12);
# }
```
[6.3]
```rust
#  fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()            
            .map(|x| x *  2)
            .filter(|x| x % 2 == 0)
            .sum();
assert_eq!(result,12);
# }
```
[6.4]
```rust
#  fn main(){
let numbers = [1,3,6];
let result: u32 = numbers
            .iter()            
            .skip(1)
            .take(1)
            .map(|x| x *  2)            
            .sum();
assert_eq!(result,12);
# }
```

---


[7] How can you pass a string to these functions?

[7.1]
```rust 
# fn main(){
let name : String= "Smith".to_string();
greet_person(name);
show_schedule(name);
# }
fn greet_person(name:String){
    println!("Hello {}",name);
}
fn show_schedule(name:String){
    println!("{}'s schedule is : A,B,C",name);
}
```
[7.2]
```rust 
# fn main(){
let name : &str = "Smith";
greet_person(name);
show_schedule(name);
# }
fn greet_person(name:String){
    println!("Hello {}",name);
}
fn show_schedule(name:String){
    println!("{}'s schedule is : A,B,C",name);
}
```
**[7.3]**
```rust 
# fn main(){
let name : &str = "Smith";
greet_person(name);
show_schedule(name);
# }
fn greet_person(name:&str){
    println!("Hello {}",name);
}
fn show_schedule(name:&str){
    println!("{}'s schedule is : A,B,C",name);
}
```
[7.4]
```rust 
# fn main(){
let name : String= String::from("Smith");
greet_person(name);
show_schedule(name);
# }
fn greet_person(name:String){
    println!("Hello {}",name);
}
fn show_schedule(name:String){
    println!("{}'s schedule is : A,B,C",name);
}
```

---

[8] Which of the following is **NOT equvalent**  to the trait bounds below?

[8.1]
```rust 
fn communicate<T: Speak + Listen>(thing: &T){
    //..
}
```
[8.2]
```rust 
fn communicate<T>(thing: &T)
    where T: Speak,
          T: Listen
{
    //..
}
```
[8.3]
```rust 
fn communicate<T>(thing: &(impl Speak + Listen)){
    //..
}
```
[8.4]
```rust 
fn communicate<T>(thing: &(Speak + Listen)){
    //..
}
```
**[8.5]**
```rust 
fn communicate<T>(thing: &T)
    where T: Speak + 
          T: Listen
{
    //..
}
```

---

[9] How can you modify as item in Vec inside a loop?

[9.1]
```rust 
# #[derive(Debug)]
# struct Items{
#    is_ordered:bool
# }
# fn main(){
#    let it1= Items{is_ordered:true};
#    let it2= Items{is_ordered:true};
#    let items: Vec<Items>= vec![it1,it2];
    for item in &mut items{
        item.is_ordered = true;
    }
# }
```
[9.2]
```rust 
# #[derive(Debug)]
# struct Items{
#    is_ordered:bool
# }
# fn main(){
#    let it1= Items{is_ordered:true};
#    let it2= Items{is_ordered:true};
#    let items: Vec<Items>= vec![it1,it2];
    for mut item in &items{
        item.is_ordered = true;
    }
# }
```
**[9.3]**
```rust 
# #[derive(Debug)]
# struct Items{
#    is_ordered:bool
# }
# fn main(){
#    let it1= Items{is_ordered:true};
#    let it2= Items{is_ordered:true};
#    let items: Vec<Items>= vec![it1,it2];
    for mut item in items{
        item.is_ordered = true;
    }
# }
```
[9.4]
```rust 
# #[derive(Debug)]
# struct Items{
#    is_ordered:bool
# }
# fn main(){
#    let it1= Items{is_ordered:true};
#    let it2= Items{is_ordered:true};
#    let items: Vec<Items>= vec![it1,it2];
    for &mut item in items{
        item.is_ordered = true;
    }
# }
```

---

[10] Which code snippet works?

[10.1]
```rust 
# #[derive(Debug)]
 enum Staff{
    CEO,
    Employee { boss: Option<Staff> } 
 }
# fn main(){
   let s= Staff:: Employee;
# }
```
[10.2]
```rust 
# #[derive(Debug)]
 enum Staff{
    CEO,
    Employee { boss: Staff } 
 }
# fn main(){
   let s= Staff:: Employee;
# }
```
[10.3]
```rust 
# #[derive(Debug)]
 enum Staff{    
    boss: Option<Staff>
 }
# fn main(){
   let s= Staff::boss;
# }
```
**[10.4]**
```rust 
# #[derive(Debug)]
struct Employee{
    boss: Staff
}
 enum Staff{    
    CEO,
    Regular(Employee),
 }
# fn main(){
   let s= Staff::Regular;
# }
```

---

[11] You have some data that needs to be processed multi-threaded. However when trying to save the data into a HashMap the compiler throws an error saying hash_map was moved. How do you synchronize and get data out of threads?

```rust 
# fn main(){
let mut hash_map: std::collections::HashMap<u32, u32>= std::collections::HashMap::new();
for i in 1..=3 {
    std::thread::spawn (move || { //error 
                    let result = i * 7;
                    hash_map.insert(i, result);
                });
}
for (i, number) in &hash_map {
    println!("{}* 7 = {}", i, number);
}
# }
```
**[11.1]**
```rust 
# fn main(){
let thread_count = 3;
let (tx, rx) = std::sync::mpsc::channel();
for i in 1..=thread_count {
    let tx = tx.clone();
    std::thread::spawn (move || {
        let result = i * 7;
        let _= tx.send((i, result));
    });
}
    for _ in 1..=thread_count {
        let (i, number) = rx.recv().unwrap();
        println!("{}* 7 = {}", i, number);
    }
# }
```
[11.2]
```rust 
# fn main(){
let mutex = std::sync::Mutex::new(std::collections::HashMap::new());
 let mut handles = vec! [];
for i in 1..3 {
    let handle = std::thread::spawn (move || {
        let result = i * 7;
        mutex.lock().unwrap().insert(i, result);
    });
    handles.push(handle);
}
for child in handles {
    let _= child.join();
}
let hash_map=mutex.lock().unwrap();
for (i, number) in &(*hash_map) {
    println!("{}* 7 = {}", i, number);
}
# }
```
[11.3]
```rust 
# fn main(){
use std::sync:: {Arc, Mutex};
let hash_map = Arc::new(Mutex::new(std::collections::HashMap::new()));
for i in 1..3 {
    let arc_map = hash_map.clone();
    std::thread::spawn (move || {
        let result = i * 7;
        arc_map.lock().unwrap().insert(i, result);
    });
}
for (i, number) in &(*hash_map.lock().unwrap()) {
    println!("{}* 7 = {}", i, number);
}
# }
```
[11.4]
```rust 
# fn main(){
let mut hash_map: std::collections::HashMap<u32, u32>= std::collections::HashMap::new();
for i in 1..=3 {
    let mut result = 0;
    std::thread::spawn (move || {
        result = i * 7;
    });
     hash_map.insert(i, result);
}
for (i, number) in &hash_map {
    println!("{}* 7 = {}", i, number);
}
# }
```
