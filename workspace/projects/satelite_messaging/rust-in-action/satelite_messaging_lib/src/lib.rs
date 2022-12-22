#![allow(dead_code, unused_variables)]

///


#[derive(Debug)]
pub struct CubeSat {
  id: u64,
}

#[derive(Debug)]
pub enum StatusMessage {
  Ok,
}

#[derive(Debug)]
pub struct Mailbox {
  pub messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Message {
    pub to: u64,
    pub content: String,
}

pub struct GroundStation {}


impl Mailbox {
    /// Mailbox.post() requires mutable access to itself and ownership over a Message.
    pub fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    /// Mailbox.deliver() requires a shared reference to a CubeSat to pull out its id field.
    /// When we find a message, returns early with the Message wrapped in Some per the Option type
    pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

impl GroundStation {
    pub fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
        }
    }
/// Calls Mailbox.post() to send messages, yielding ownership of a Message
    pub fn send(&self, mailbox: &mut Mailbox,msg: Message) {
         mailbox.post(msg);
    }
}
/// Calls Mailbox.deliver() to receive messages, gaining ownership of a Messag
impl CubeSat {
    pub fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}


/// Returns a vector of CubeSat IDs
pub fn fetch_sat_ids() -> Vec<u64> {
  vec![1,2,3]
}
