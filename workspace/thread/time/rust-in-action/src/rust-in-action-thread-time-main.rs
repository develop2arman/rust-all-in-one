#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-time_bin --bin rust-in-action-thread-time-main```
///
/// ```cargo doc  --package rust-in-action-thread-time_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-time_bin```
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
///
/// ```compile_fail,ignore
use std::time;
use std::f64::consts::PI;
use std::thread;
fn main() {
    let mut sensor_array_state: f64 = 1.0;

    for step in 1.. {
        pause(step);
        let reading = take_reading(sensor_array_state);
        print_reading(reading);
        sensor_array_state += 0.05;
    }
}
fn pause(step: usize) {
  let delay_ms = if step % 4 == 0 {
      500
  } else if step % 7 == 0 {
      1000
  } else {
      3
  };
  let delay = time::Duration::from_millis(delay_ms);
  thread::sleep(delay);
}
fn take_reading(x: f64) -> f64 {
  // sin(x) + sin(π / sin(πx))
  x.sin()   + (PI / (x*PI).sin()).sin()
}
fn print_reading(reading: f64) {
  println!("{}", reading);
}
