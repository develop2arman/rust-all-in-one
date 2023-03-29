#![crate_name = "rust_all_in_one"]
#![allow(dead_code, unused_variables)]

#![doc(html_logo_url = "https://armanriazi.github.io/site/assets/attachments/me.png")]


/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-all-in-one```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
///
/// ```cargo clippy  --workspace --message-format=short --all-targets```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// ```rust,compile_fail,ignore
/// #[allow(dead_code)]
/// fn m(){
///     core::panicking::panic!("in crash");
///     unimplemented!()
/// }
/// ```

#[cfg(panic = "unwind")]
#[cfg(target_family = "unix")]
fn get_platform() -> String {
    "UNIX".into()
}

#[cfg(target_family = "windows")]
fn get_platform() -> String {
    "Windows".into()
}

fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

/// We have two get_platform and selected by conditional_features.
/// > Line 11 :(commented)
/// /// error: unneeded unit return type ^^^^ help: remove the `-> ()`
/// fn nothing()->(){
/// }

fn finish() -> impl std::process::Termination {
    let machine_kind = if cfg!(unix) {
        println!("I was running on a unix machine!");
        std::process::ExitCode::SUCCESS
    } else if cfg!(windows) {
        println!("I was running on a windows machine!");
        std::process::ExitCode::SUCCESS
    } else {
        println!("I was running on a unknown machine!");
        std::process::ExitCode::FAILURE
    };
}

// fn main(){
//     greet_world();
//     //
// }
/* 1
fn main() {
let mut favorite_foods = vec! ["potato", "tomato"];
let mut healthy_foods = &mut favorite_foods;
healthy_foods.push("carrot");
let mut grocery_list = &mut favorite_foods;
grocery_list.push("cookies");
println! ("Healthy Foods: "); for food in healthy_foods { println!("{}", food);
}
println! ("Grocery List:"); for food in grocery_list { println!("{}", food);
}

}
//error[E0499]: cannot borrow `favorite_foods` as mutable more than once at a time
*/

/* 2
fn main() {
fn first_word<'a>(sentence: &'a str, separator: &'a str) -> &'a str {
    let word: &'a str = sentence
.split(separator)
.next()
.unwrap();
word
}
}
*/

/* 3
fn main(){

    use std::collections::HashMap;
    let word = "internationalization";
    let mut letter_count = HashMap::new();


    for letter in word.chars() {
        // 1
        let count = letter_count.entry (letter).or_insert(0);
        *count += 1;
        // 2
        //    match &mut letter_count.get(&letter) {
        //        Some (count) => *count += 1,
        //}
        // => {letter_count.insert(letter, 1);},
        //}
        // 3
        // if let Some (count)
        //    letter_count.get(&letter) {
        //        letter_count.insert(letter, count + 1);
        //    } else {
        // }

        //letter_count.insert(letter, 1);


    }
    for (letter, count) in &letter_count {
        println!("{}: {}", letter, count);
    }
}
*/

/*4
fn main(){

let mut vacation_spots= vec! ["New York City", "Yosemite", "Monterey"];
let handle = std::thread::spawn( move || {
    for spot in vacation_spots {
        println!("{}", spot);
    }
    });
    vacation_spots.remove(1);
    handle.join().unwrap();
}
*/

/*
fn main(){
let mut echo=std::process::Command::new("echo")
    .arg("one two three")
    .stdout(Stdio::piped())
    .spawn()?;
let wc = std::process::Command::new("wc")
    .arg("-w")
    .stdin(Stdio::piped())
    .stdout(stdio::inherit())
    .spawn()?;
let mut wc_in= let echo_out&mut wc.stdin.ok_or_else(|| Error::from(ErrorKind::BrokenPipe))?;
echo.stdout.as_mut().ok_or_else(|| Error::from(ErrorKind::BrokenPipe))?;
io::copy(echo_out, &mut wc_in)?;

}

```
1. Pipes echo's stdout to the program's stdin.
2. Pipes the program's stdin into wc.
3. Copies echo's stdout into wc's stdin.
//
1. Starts echo.
2. Pipes wc's stdout to the program's stdout.
3. Copies echo's stdout into wc's stdin.
//
1. Pipes the program's stdin into echo.
2. Pipes wc's stdout to the program's stdin.
3. Copies echo's stdout into wc's stdin.
//
1. Starts echo.
2. Starts wc.
3. Copies echo's stdout into wc's stdin.
4. Nothing is printed to the console.
```
*/



 fn main(){

unimplemented!();
}
