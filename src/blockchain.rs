use crate::hash_string;
use chrono;

pub struct Block {
  from_address: String,
  to_address: String,
  timestamp: String,
  ammount: u64,
  nonce: u64,
  hash: String
}

pub trait Printer {
  fn print(&self) -> String;
}

impl Block {
  pub fn new(
      from_address: String, to_address: String,
      ammount: u64) -> Block {
    let mut b: Block = Block {
      from_address : from_address,
      to_address : to_address,
      timestamp : format!("{:?}", chrono::offset::Utc::now()),
      ammount : ammount,
      nonce : 0,
      hash: String::new()
    };
    b.calcHash();
    return b;
  }

  pub fn calcHash(&mut self) {
    let mut s: String = String::new();
    s.push_str(&self.to_address);
    s.push_str(&self.from_address);
    s.push_str(&self.timestamp);
    s.push_str(&self.nonce.to_string());
    s.push_str(&self.ammount.to_string());
    self.hash = hash_string(&s);
  }

}

impl Printer for Block {
  fn print(&self) -> String {
    return format!(
      "[From: {}, To: {}, Ammount: {}, Timestamp: {}, \
Nonce: {}, Hash: {}]",
      self.from_address, self.to_address, self.ammount,
      self.timestamp, self.nonce, self.hash);
  }
}

