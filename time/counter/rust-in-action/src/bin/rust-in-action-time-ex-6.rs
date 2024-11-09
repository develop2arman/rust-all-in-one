#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-time_bin --bin rust-in-action-time-ex-6```
///
/// ```cargo doc  --package rust-in-action-time_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-time_bin```
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
/// // ```compile_fail,ignore
use std::time;

struct UsageCounter {
  count: usize,
  reset_interval: time::Duration,
  last_reset_at: time::SystemTime,
}

impl UsageCounter {
    fn incr(&mut self) {
      self.count += 1;
    }

    fn reset(&mut self) {
      self.count = 0;
      self.last_reset_at = time::SystemTime::now();
    }

    fn check_usage(&mut self) -> (time::SystemTime, usize) {
      let since = self.last_reset_at;
      let usage = self.count;
      let now = time::SystemTime::now();

      let time_for_next_reset = since + self.reset_interval;
      if time_for_next_reset < now {
        self.reset();
      }

      (since, usage)
    }
}

fn main() {
  let start = time::SystemTime::now();
  let hourly = time::Duration::from_secs(60 * 60);

  let usage_counters = [
    UsageCounter { count:0, reset_interval: hourly, last_reset_at: start},
    UsageCounter { count:0, reset_interval: hourly, last_reset_at: start},
    UsageCounter { count:0, reset_interval: hourly, last_reset_at: start},
  ];

  run(usage_counters);
}

fn run(mut usage_counters: [UsageCounter; 3]) {
  for _ in 0..50 {
    for uc in &mut usage_counters {
      uc.incr();
    }
  }

  usage_counters[1].reset();

  for uc in &mut usage_counters {
    let (_reset_at, count) = uc.check_usage();
    println!{"{}", count};
  }
}
