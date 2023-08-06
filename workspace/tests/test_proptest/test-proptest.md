
> `tags` [[test_proptest]] [[test]]

## Proptest
Proptest is a property testing framework (i.e., the QuickCheck family) inspired by the Hypothesis framework for Python. It allows to test that certain properties of your code hold for arbitrary inputs, and **if a failure is found,**  *automatically finds the minimal test case to reproduce the problem*. **Unlike QuickCheck, generation and shrinking is defined on a per-value basis instead of per-type**, which makes it more flexible and simplifies composition.

## What is property testing?
Property testing is a system of testing code by checking that certain properties of its output or behaviour are fulfilled for all inputs.
Property testing is best used to complement traditional unit testing (i.e., using specific inputs chosen by hand). *Traditional tests can test specific known edge cases, simple inputs, and inputs that were known in the past to reveal bugs,* **whereas property tests will search for more complicated inputs that cause problems.**

## Property Pattern 
Now we can add some property tests to our date parser. But *how do we test the date parser for arbitrary inputs, without making another date parser in the test to validate it?* We won't need to as long as we choose our inputs and properties correctly. 
> But **before correctness**, there's actually an even simpler property to test: **The function should not crash.** Let's start there.

```
// Bring the macros and other important things into scope.
use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        parse_date(&s);
    }
}
```

What this does is take a literally random &String (ignore \\PC* for the moment, we'll get back to that — if you've already figured it out, contain your excitement for a bit) and give it to parse_date() and then throw the output away.

When we run this, we get a bunch of scary-looking output, eventually ending with

```
thread 'main' panicked at 'Test failed: byte index 4 is not a char boundary; it is inside 'ௗ' (bytes 2..5) of `aAௗ0㌀0`; minimal failing input: s = "aAௗ0㌀0"
	successes: 102
	local rejects: 0
	global rejects: 0
'
```

If we look at the top directory after the test fails, we'll see a new proptest-regressions directory, which contains some files corresponding to source files containing failing test cases. These are failure persistence files. The first thing we should do is add these to source control.

```
$ git add proptest-regressions
```
The next thing we should do is copy the failing case to a traditional unit test since it has exposed a bug not similar to what we've tested in the past.

```
#[test]
fn test_unicode_gibberish() {
    assert_eq!(None, parse_date("aAௗ0㌀0"));
}
```
Now, let's see what happened... we forgot about UTF-8! You can't just blindly slice strings since you could split a character, in this case that Tamil diacritic placed atop other characters in the string.

In the interest of making the code changes as small as possible, we'll just check that the string is ASCII and reject anything that isn't.

```
fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() { return None; }

    // NEW: Ignore non-ASCII strings so we don't need to deal with Unicode.
    if !s.is_ascii() { return None; }

    if "-" != &s[4..5] || "-" != &s[7..8] { return None; }

    let year = &s[0..4];
    let month = &s[6..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(
        |y| month.parse::<u32>().ok().and_then(
            |m| day.parse::<u32>().ok().map(
                |d| (y, m, d))))
}
```

The tests pass now! But we know there are still more problems, so let's test more properties.

Another property we want from our code is that it parses every valid date. We can add another test to the proptest! section:

```
proptest! {
    // snip...

    #[test]
    fn parses_all_valid_dates(s in "[0-9]{4}-[0-9]{2}-[0-9]{2}") {
        parse_date(&s).unwrap();
    }
}
```

The thing to the right-hand side of in is actually a regular expression, and s is chosen from strings which match it. So in our previous test, "\\PC*" was generating arbitrary strings composed of arbitrary non-control characters. Now, we generate things in the YYYY-MM-DD format.

The new test passes, so let's move on to something else.

The final property we want to check is that the dates are actually parsed correctly. Now, we can't do this by generating strings — we'd end up just reimplementing the date parser in the test! Instead, we start from the expected output, generate the string, and check that it gets parsed back.

```
proptest! {
    // snip...

    #[test]
    fn parses_date_back_to_original(y in 0u32..10000,
                                    m in 1u32..13, d in 1u32..32) {
        let (y2, m2, d2) = parse_date(
            &format!("{:04}-{:02}-{:02}", y, m, d)).unwrap();
        // prop_assert_eq! is basically the same as assert_eq!, but doesn't
        // cause a bunch of panic messages to be printed on intermediate
        // test failures. Which one to use is largely a matter of taste.
        prop_assert_eq!((y, m, d), (y2, m2, d2));
    }
}
```

Here, we see that besides regexes, we can use any expression which is a proptest::strategy::Strategy, in this case, integer ranges.

The test fails when we run it. Though there's not much output this time.

```
thread 'main' panicked at 'Test failed: assertion failed: `(left == right)` (left: `(0, 10, 1)`, right: `(0, 0, 1)`) at examples/dateparser_v2.rs:46; minimal failing input: y = 0, m = 10, d = 1
	successes: 2
	local rejects: 0
	global rejects: 0
', examples/dateparser_v2.rs:33
note: Run with `RUST_BACKTRACE=1` for a backtrace.

The failing input is (y, m, d) = (0, 10, 1), which is a rather specific output. Before thinking about why this breaks the code, let's look at what proptest did to arrive at this value. At the start of our test function, insert

println!("y = {}, m = {}, d = {}", y, m, d);
```