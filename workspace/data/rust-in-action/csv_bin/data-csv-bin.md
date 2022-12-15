 ## `Concepts`

 ## Higher-order programming

Functions can both accept and return functions. includes a [[CLOSURE]], also known as an anonymous function or #lambda function.

 ### Examples
 
 ```
 let fields: Vec<_> = record
.split(',')
.map(|field| field.trim())
.collect();
 ```

 ## Conditional compilation

Not included in release builds of the program.
[[conditional_compilation]]
#cfg

 ### Examples

 ```
 if cfg!(debug_assertions) {
 eprintln!("debug: {:?} -> {:?}",record, fields);
}
 ```

 ## Conditionally processing data

The if let construct is a concise method of conditionally processing data that also provides a local variable assigned to that data. The parse() method returns Ok(T) (where T stands for any type) when it can successfully parse the string; otherwise, it returns Err(E) (where E stands for an error type)

 ### Examples
 ```
 if let Ok(length) = fields[1].parse::<f32>() {     
 }
 ```

tags #debug #map
