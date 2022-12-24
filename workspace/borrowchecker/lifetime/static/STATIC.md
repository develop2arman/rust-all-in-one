
[[re-lifetime-static]]

[[ria-lifetime-static]]

[[pb-lifetime-static]]

## Struct
Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.
In addition, the lifetime elision rules often make it so that **lifetime annotations aren’t necessary in method signatures**.
#struct


```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
   

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
```
    //no_err_func
fn first_word(s: &str) -> &str {

fn first_word<'a>(s: &'a str) -> &str {

fn first_word<'a>(s: &'a str) -> &'a str {

fn longest(x: &str, y: &str) -> &str {
   //err_func
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
  //no_err_method
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

tags
