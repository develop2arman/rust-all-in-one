

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

**[9.1]**
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
[9.3]
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
    Regular(Empolyee),
 }
# fn main(){
   let s= Staff::Regular;
# }
```

---

[11] How can you modify as item in Vec inside a loop?

[11.1]
```rust 
# fn main(){

# }
```
