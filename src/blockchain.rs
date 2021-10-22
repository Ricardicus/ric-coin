use crate::hash_string;

pub struct Block {
  from_address: String,
  to_address: String,
  ammount: u64
}

pub trait CalculateHash {
  fn hash(&self) -> String;
}

impl Block {
  pub fn new(from_address: String, to_address: String,
         ammount: u64) -> Block {
    Block {
      from_address : from_address,
      to_address : to_address,
      ammount : ammount
    }
  }
}

impl CalculateHash for Block {
  fn hash(&self) -> String {
    let mut s = String::new();
    s.push_str(&self.to_address);
    s.push_str(&self.from_address);
    s.push_str(&self.ammount.to_string());
    return hash_string(&s);
  }
}

