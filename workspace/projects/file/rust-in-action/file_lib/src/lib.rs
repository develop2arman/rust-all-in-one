#![allow(dead_code, unused_variables)]
//! Simulating files one step at a time.
use std::io::prelude::*;
use std::fmt;  // <2> Bring the `std::fmt` crate into local scope, allowing us to make use of `fmt::Result`
use std::fmt::{Display};  // <3> Bring `Display` into local scope, avoiding the need for us to prefix it as `fmt::Display` in our code


#[derive(Debug,PartialEq)]
pub enum FileState {
   Open,
   Closed,
 }

 /// Represents a "file",
 /// which probably lives on a file system.
 #[derive(Debug,PartialEq)]
 pub struct File {
   pub name: String,
   data: Vec<u8>,
   pub state: FileState
 }

 impl Display for FileState {
  fn fmt(&self, f:
         &mut fmt::Formatter,
  ) -> fmt::Result {
    match *self {
      FileState::Open => write!(f, "OPEN"),
      FileState::Closed => write!(f, "CLOSED"),
    }
  }
}

impl Display for File {
   fn fmt(&self, f:
          &mut fmt::Formatter,
   ) -> fmt::Result {
      write!(f, "<{} ({})>",
             self.name, self.state)
   }
}

pub trait Read {
  fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

/// # Notice
/// `save_to.append(&mut tmp);`
/// after append tmp vec will be empty
///
#[allow(unused_variables)]
impl Read for File {
  fn read(
    self: &File,
    save_to: &mut Vec<u8>,
  ) -> Result<usize, String> {
    if self.state != FileState::Open {
      return Err(String::from("File must be open for reading"));
    }
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
  }
}

impl File {
     /// Creates a new, empty `File`.
   ///
   /// # Examples
   ///
   /// ```no_compile
   /// let f = File::new("f1.txt");
   /// ```
     /// New files are assumed to be empty, but a name is required.
   pub fn new(name: &str) -> File {
     File {
       name: String::from(name),
       data: Vec::new(),
       state: FileState::Closed,
     }
   }
    pub fn new_with_data(name: &str,data: &Vec<u8>) -> File {

      let mut f = File::new(name);

      f.data = data.clone();

      f
   }
   /// Returns the file's length in bytes.
   pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Returns the file's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
 }



 pub fn open(mut f: File) -> Result<File, String> {
   f.state = FileState::Open;
   Ok(f)
 }

 pub fn close(mut f: File) -> Result<File, String> {
   f.state = FileState::Closed;
   Ok(f)
 }
