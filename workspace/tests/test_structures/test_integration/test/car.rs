#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p test_integration  car -- --nocapture```
///
/// ```cargo doc  --package test_integration --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package test_integration```
///
/// ## What
/// `Test Integration Structure`
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
/// `nothing`
///
use lib::car::PublicCar;

#[test]
fn test_public_car() {
    let car = PublicCar {
        owner: "Ylvis Stonehenge".to_string(),
        license: "ABC123".to_string(),
        model: "Civic".to_string(),
        make: "Honda".to_string(),
        year: 2013,
    };
    assert_eq!(car.owner, "Ylvis Stonehenge");
    assert_eq!(car.license, "ABC123");
    assert_eq!(car.model, "Civic");
    assert_eq!(car.make, "Honda");
    assert_eq!(car.year, 2013);
}
