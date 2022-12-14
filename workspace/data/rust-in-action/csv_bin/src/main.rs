#![allow(dead_code, unused_variables)]
#![allow(dead_code, unused_variables)]

/// # Commands
///
/// ```cargo run -p csv_bin_bin -q```
/// 
/// # Concepts
///
/// * `Higher-order programming`
///
///! Functions can both accept and return functions. includes a #closure, also known as an anonymous function or #lambda function.
///
/// # Examples
/// ```
/// let fields: Vec<_> = record
///.split(',')
///.map(|field| field.trim())
///.collect();
/// ```
///
/// * `Conditional compilation`
///
///! Not included in release builds of the program.
/// # Examples
/// ```
/// if cfg!(debug_assertions) {
/// eprintln!("debug: {:?} -> {:?}",record, fields);
///}```
/// ```
///
/// * `Conditionally processing data`
///
///! The if let construct is a concise method of conditionally processing data that also provides a local variable assigned to that data. The parse() method returns Ok(T) (where T stands for any type) when it can successfully parse the string; otherwise, it returns Err(E) (where E stands for an error type)
/// # Examples
/// ```
/// if let Ok(length) = fields[1].parse::<f32>() {
/// ...
/// }
/// ```
///
fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
      if i == 0 || record.trim().len() == 0 {
        continue;
      }

      let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
      if cfg!(debug_assertions) {
        eprintln!("debug: {:?} -> {:?}",
               record, fields);
      }

      let name = fields[0];

      if let Ok(length) = fields[1].parse::<f32>() {
          println!("{}, {}cm", name, length);
      }
    }
  }
