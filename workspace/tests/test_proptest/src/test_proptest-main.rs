#![allow(unused)]
#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p test_proptest_bin --bin test_proptest-main```
///
/// ```cargo doc  --package test_proptest_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package test_proptest_bin```
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
///  `TODO`
///

use std::ascii::*; //NOREADME
// NOREADME
fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() { return None; }

    // NEW: Ignore non-ASCII strings so we don't need to deal with Unicode.
    //if !s.is_ascii() { return None; } // commented to create file log by using doesnt_crash
    // uncomment it to pass all test after created file

    if "-" != &s[4..5] || "-" != &s[7..8] { return None; }

    let year = &s[0..4];
    let month = &s[6..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(
        |y| month.parse::<u32>().ok().and_then(
            |m| day.parse::<u32>().ok().map(
                |d| (y, m, d))))
}



#[cfg(test)]
mod tests {
use super::*;
use proptest::prelude::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(None, parse_date("2017-06-1"));
        assert_eq!(None, parse_date("2017-06-170"));
        assert_eq!(None, parse_date("2017006-17"));
        assert_eq!(None, parse_date("2017-06017"));
        assert_eq!(Some((2017, 06, 17)), parse_date("2017-06-17"));
    }

    #[test]
    fn test_october_first() {
        assert_eq!(Some((0, 10, 1)), parse_date("0000-10-01"));
    }

    #[test]
    fn test_unicode_gibberish() {
        assert_eq!(None, parse_date("0 𞺫A® "));
    }

    proptest! {
        // What this does is take a literally random &String (ignore \\PC* for the moment, we’ll get back to that — if you’ve already figured it out, contain your excitement for a bit) and give it to parse_date() and then throw the output away.
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            parse_date(&s);
        }

        // Another property we want from our code is that it parses every valid date. We can add another test to the proptest! section:
        #[test]
        fn parses_all_valid_dates(s in "[0-9]{4}-[0-9]{2}-[0-9]{2}") {
            parse_date(&s).unwrap();
        }
   
    }
}

fn main() {unimplemented!()}