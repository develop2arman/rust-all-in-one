#![allow(dead_code, unused_variables)]




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
  messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Message {
    to: u64,
    content: String,
}

pub struct GroundStation {}

impl Mailbox {
    pub fn post(&mut self, to: &CubeSat, msg: Message) {
        self.messages.push(msg);
    }

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
    pub fn connect(&self, sat_id: &u64) -> CubeSat {
        CubeSat {
            id: *sat_id,
        }
    }

    pub fn send(&self, mailbox: &mut Mailbox, to: &CubeSat, msg: Message) {
        mailbox.post(to, msg);
    }
}

impl CubeSat {
    pub fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}


/// Returns a vector of CubeSat IDs
pub fn fetch_sat_ids() -> Vec<u64> {
  vec![1,2,3]
}
