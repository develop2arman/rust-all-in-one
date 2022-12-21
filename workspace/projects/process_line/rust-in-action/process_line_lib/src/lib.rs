#![allow(dead_code, unused_variables)]
use std::io::prelude::*;
use regex::Regex;


pub fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
        Some(_) => println!("{}", line),
        None => (),
        }
    }
 }
