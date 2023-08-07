use self::common::adder;

/// process test
///
/// ## Commands
///
///
/// ```cd test_dependent```
/// 
/// ```cargo test  -p test_dependent -- --show-output```
///
/// ```cargo test  -p test_dependent -- --show-output --ignore```
///             
/// ```cargo doc  --package test_dependent --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package test_dependent```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `TODO`
///
/// # Return
/// 
/// `passed`
/// 

mod common;

#[cfg(test)]
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
    common::teardown();
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}