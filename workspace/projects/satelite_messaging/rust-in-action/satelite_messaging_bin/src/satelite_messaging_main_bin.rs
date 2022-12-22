#![allow(dead_code, unused_variables)]

use satelite_messaging_lib::*;
/// Main
///
/// ## Commands
///
/// ```cargo run  -- arman  -- -```
///
///```cargo run  -- arman  -- readme.txt```
///
/// ```cargo run -q -p satelite_messaging_bin --bin satelite_messaging_main_bin```
///
/// ```cargo doc  --package satelite_messaging_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package satelite_messaging_bin```
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
/// `CubeSat { id: 1 }: Some(Message { to: 1, content: "hello" })`
/// `CubeSat { id: 2 }: Some(Message { to: 2, content: "hello" })`
/// `CubeSat { id: 3 }: Some(Message { to: 3, content: "hello" })`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// // ```compile_fail,ignore
///


 fn main() {
     let mut mail = Mailbox { messages: vec![] };

     let base = GroundStation {};

     let sat_ids = fetch_sat_ids();

     for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message { to: sat_id, content: String::from("hello") };
        base.send(&mut mail, msg);
     }

     let sat_ids = fetch_sat_ids();

     for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
     }

}
// fn main() {
//     let base = GroundStation {};
//     let mut sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] } };
//     let mut sat_b = CubeSat { id: 1, mailbox: Mailbox { messages: vec![] } };
//     let mut sat_c = CubeSat { id: 2, mailbox: Mailbox { messages: vec![] } };

// }
