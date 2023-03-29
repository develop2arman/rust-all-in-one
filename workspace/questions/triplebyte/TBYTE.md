[1] resolve the problem?

[1.x]
```rust 
#  fn main(){
let mut favorite_foods = vec! ["potato", "tomato"];
let mut healthy_foods = &mut favorite_foods;
healthy_foods.push("carrot");
let mut grocery_list = &mut favorite_foods;//error[E0499]: cannot borrow `favorite_foods` as mutable more than once at a time
grocery_list.push("cookies");
println! ("Healthy Foods: ");
for food in healthy_foods {
    println!("{}", food);
}
println! ("Grocery List:");
for food in grocery_list {
    println!("{}", food);
}
# }
```
---

[2] resolve the problem?

**[2.1]**
```rust 
#  fn main(){
fn first_word<'a>(sentence: &'a str, separator: &'a str) -> &'a str {
    let word: &'a str = sentence
                            .split(separator)
                            .next()
                            .unwrap();
    word
}

println!("{}",first_word("arman riazi"," "));

# }
```
[2.2]
```rust 
#  fn main(){
fn first_word(sentence: &str, separator: &str) -> &str {
    let word: &'a str = sentence
                            .split(separator)
                            .next()
                            .unwrap();
    word
}

println!("{}",first_word("arman riazi"," "));

# }
```
[2.3]
```rust 
#  fn main(){
    fn first_word<'a>(sentence: &str, separator:  &str) -> &'a str {
    let word: &str = sentence
                            .split(separator)
                            .next()
                            .unwrap();
        word
    }
    println!("{}",first_word("arman riazi"," "));
# }
```
[2.4]
```rust 
#  fn main(){
    fn first_word<'a>(sentence: &'a str, separator:  &'a str) -> &str {
    let word: &str = sentence
                            .split(separator)
                            .next()
                            .unwrap();
        word
    }
    println!("{}",first_word("arman riazi"," "));
# }
```

---

[3] Count of array?

**[3.1]**
```rust 
#  fn main(){
    use std::collections::HashMap;
    let word = "internationalization";
    let mut letter_count = HashMap::new();

    for letter in word.chars() {

        let count = letter_count.entry (letter).or_insert(0);
        *count += 1;
    }
    for (letter, count) in &letter_count {
        println!("{}: {}", letter, count);
    }
# }
```
[3.2]
```rust 
#  fn main(){
    use std::collections::HashMap;
    let word = "internationalization";
    let mut letter_count = HashMap::new();

    for letter in word.chars() {
            match &mut letter_count.get(&letter) {
                Some (count) => *count += 1,
        }
         => {letter_count.insert(letter, 1);},
    }    
    for (letter, count) in &letter_count {
        println!("{}: {}", letter, count);
    }
# }
```
[3.3]
```rust 
#  fn main(){
    use std::collections::HashMap;
    let word = "internationalization";
    let mut letter_count = HashMap::new();

    for letter in word.chars() {                    
        if let Some (count) = letter_count.get(&letter) {
               letter_count.insert(letter, count + 1);
           } else {
            letter_count.insert(letter, 1);
        }        
    }
    for (letter, count) in &letter_count {
        println!("{}: {}", letter, count);
    }
# }
```
[3.4]
```rust 
#  fn main(){
    use std::collections::HashMap;
    let word = "internationalization";
    let mut letter_count = HashMap::new();

    for letter in word.chars() {                    
      match &mut letter_count.get(&letter){
        Some(count)=> *count +=1,
        _ => {letter_count.insert(letter,1);}
      }
    }
    for (letter, count) in &letter_count {
        println!("{}: {}", letter, count);
    }
# }
```


[4] Count of array?

```rust 
#  fn main(){
let mut vacation_spots= vec! ["New York City", "Yosemite", "Monterey"];
let handle = std::thread::spawn( move || {
    for spot in vacation_spots {
        println!("{}", spot);
    }
    });
    vacation_spots.remove(1);// Error value borrowed here after move
    handle.join().unwrap();
# }
```
[4.1] The value vacation_spots was moved and then mutated in another thread. This helps us prevent race conditions.
[4.2] Without the move keyword, the join().unwrap() would always panic. This helps us write code that does not panic.
[4.3] Without the move keyword, the compiler wouldn't know the thread might use vacation_spots after it was mutated. This helps us prevent borrowing after moving values.
[4.4] The closure is moved. This helps us make sure the closure used only once.

---

[5] Stdio?
```rust 
#  fn main(){
use std::process::Stdio;
use std::{io::ErrorKind, error::Error};
//unimplemented!();

let mut echo=std::process::Command::new("echo")
    .arg("one two three")
    .stdout(Stdio::piped())
    .spawn()?;

let wc = std::process::Command::new("wc")
    .arg("-w")
    .stdin(Stdio::piped())
    .stdout(Stdio::inherit())
    .spawn()?;

let mut wc_in= true;
let echo_out=&mut wc.stdin.ok_or_else(|| Error::from(ErrorKind::BrokenPipe))?;

echo.stdout.as_mut().ok_or_else(|| Error::from(ErrorKind::BrokenPipe))?;

std::io::copy(echo_out, &mut wc_in)?;
# }
```
[5.1]
1. Pipes echo's stdout to the program's stdin. 
2. Pipes the program's stdin into wc.
3. Copies echo's stdout into wc's stdin.
[5.2]
1. Starts echo.
2. Pipes wc's stdout to the program's stdout. 
3. Copies echo's stdout into wc's stdin.
[5.3]
1. Pipes the program's stdin into echo. 
2. Pipes wc's stdout to the program's stdin. 
3. Copies echo's stdout into wc's stdin.
[5.4]
1. Starts echo.
2. Starts wc.
3. Copies echo's stdout into wc's stdin. 
4. Nothing is printed to the console.

---

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

---

[12] What is the output this puzzle?

```rust 
# fn main(){
let a = Some("a");
let b = Some("b");
let c = Some("c");
let d = Some("d");
let a_or_d = a.unwrap_or (d.unwrap());
//println!("a_or_d  {}", a_or_d.clone()); //return a
let default_not = || b.iter().next(); //return NONE
let c_mapped = |_| Some(&a_or_d);
let result= c.map_or_else(default_not, c_mapped).unwrap();
println!("Result {}", result);
# }
```
[12.1]
**a**
[12.2]
b
[12.3]
c
[12.4]
d


---

[13] There is a logging function called log that takes a message: &str argument. You need to use this function to print "123". Which of the following methods work?

[13.1]
```rust 
# fn main(){

fn log(message: &str) {
println!("{}", message);
}

let numbers = "123";
log(&numbers[0..4]);
# }
```
[13.2]
```rust 
# fn main(){

fn log(message: &str) {
println!("{}", message);
}

let numbers = "0123";
log(&numbers[1:4]);
# }
```
**[13.3]**
```rust 
# fn main(){

fn log(message: &str) {
println!("{}", message);
}

let numbers = "1234";
log(&numbers[..3]);
# }
```
**[13.4]**
```rust 
# fn main(){

fn log(message: &str) {
println!("{}", message);
}

let numbers = "01234";
log(&numbers[1..4]);
# }
```
---

[14] You want to print the status of a request in the following format:

> `success: true, errors: 0, message: success`

> Which of the following is **NOT valid Rust**?

[14.1]
```rust 
# fn main(){
let result = (true, 0, "success");
let (success, errors, message) = result;
println!("success: {}, errors: {}, message: {}", success, errors, message);
# }
```
[14.2]
```rust 
# fn main(){
let result: (bool, i32, &str) = (true, 0, "success");
println! ("success: {}, errors: {}, message: {}", result.0, result.1, result.2);
# }
```
[14.3]
```rust 
# fn main(){
let result = (true, 0, "success");
println! ("success: {}, errors: {}, message: {}", result.0, result.1, result.2);
# }
```
**[14.4]**
```rust 
# fn main(){
let result = [true, 0, "success"];
println! ("success: {}, errors: {}, message: {}", result[0], result[1], result[2]);
# }
```

---

[15] went on vacation, and the lead developer is not happy with the work they delivered below. The lead says the code "crashes all over the place, hides the issue from the calling function, and panics unrecoverably. They needed an expert so they called you in to fix it. Which function will you deliver?

```rust 
# fn main(){
fn export_todo (filename: &str, todo_list: &[&str], done_list: &[&str]) {
    let mut file= match std::fs::File::create(filename) {
        Err(e) => panic!("{}", e),
        Ok (f) => f,
    };

    match file.write_all( b"# To Do List\n") {
        Err(e) => panic!("{}", e),
        Ok (()) => (),
    }
    match file.write_all(b"## Next\n") {
        Err(e) => panic!("{}", e),
        Ok(()) => (),
    }
    for item in todo_list.iter() {
        match file.write_all(format! ("- [] {}\n", item).as_bytes()) {
            Err(e) => panic!("{}", e),
            Ok (()) => (),
        }
    }
}

# }
```
**[15.x]**
```rust  
# fn main(){
    use std::io::Write;
    //Because of return std::io::Result<()> is a good answer. anyoption that do not have Result will not be answer
    fn export_todo(filename: &str, todo_list: &[&str], done_list: &[&str]) -> std::io::Result<()> {

        let mut file = std::fs::File::create(filename)?;
        file.write_all(b"# To Do List \n")?;
        file.write_all(b"## Next\n")?;

        for item in todo_list.iter() {
            file.write_all(format! ("- [ ] {}\n", item).as_bytes())?
        }

        file.write_all(b"## Done\n")?;
        for item in done_list.iter() {
            file.write_all(format! ("- [x] {}\n", item).as_bytes())?
        }
    Ok(())
    }
# }
```
