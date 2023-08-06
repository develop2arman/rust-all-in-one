use self::common::adder;
/// process test
///
/// ## Commands
///
///
/// ```cd rust-survey-json_test```
/// 
/// ```cargo test  -p test_independent -- --show-output```
///
/// ```cargo test  -p test_independent-- --show-output --ignore```
///             
/// ```cargo doc  --package integration_test--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-survey-json_test```
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
}


#[test]
#[ignore]
fn expensive_test() {
    //common::setup();
    assert_eq!(7, 7);
}
