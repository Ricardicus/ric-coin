use crate::keygenerator::{hash_string, KeyMaster};
use chrono;

pub struct Transaction {
  from_address: String,
  to_address: String,
  ammount: u64,
  timestamp: String,
  hash: String,
  signature: String,
  public_key: String
}

pub struct Block {
  timestamp: String,
  nonce: u64,
  hash: String,
  previous_hash: String,
  transactions: Vec<Transaction>
}

pub struct Blockchain {
  blocks: Vec<Block>
}

pub trait Printer {
  fn print(&self) -> String;
}

impl Transaction {
  pub fn new(
      keys: &KeyMaster,
      from_address: String,
      to_address: String,
      ammount: u64) -> Transaction {
    let mut t: Transaction = Transaction {
      timestamp : format!("{:?}", chrono::offset::Utc::now()),
      ammount : ammount,
      from_address : from_address,
      to_address : to_address,
      hash : String::new(),
      signature: String::new(),
      public_key: String::new()
    };
    t.calc_hash();
    t.signature = keys.sign(t.get_hash().to_string());
    t.public_key = keys.public_key.to_string();
    return t;
  }

  pub fn calc_hash(&mut self) {
    let mut s: String = String::new();
    s.push_str(&self.from_address);
    s.push_str(&self.to_address);
    s.push_str(&self.ammount.to_string());
    s.push_str(&self.timestamp);
    s.push_str(&self.public_key);
    self.hash = hash_string(&s);
  }

  pub fn get_hash(&self) -> &str {
    return &self.hash[..];
  }
}

impl Block {
  pub fn new() -> Block {
    let mut b: Block = Block {
      timestamp : format!("{:?}", chrono::offset::Utc::now()),
      nonce : 0,
      hash: String::new(),
      previous_hash: String::new(),
      transactions: Vec::<Transaction>::new()
    };
    b.calc_hash();
    return b;
  }

  pub fn add_transaction(&mut self, transaction: Transaction) {
    self.transactions.push(transaction);
  }

  pub fn get_transactions(&self) -> String {
    let mut s: String = String::new();
    s.push_str("[");
    for transaction in self.transactions.iter() {
      let t: String = transaction.print();
      if s.len() > 1 {
        s.push_str(",");
      }
      s.push_str(&t);
    }
    s.push_str("]");
    return s;
  }

  pub fn calc_hash(&mut self) {
    let mut s: String = String::new();
    s.push_str(&self.get_transactions());
    s.push_str(&self.timestamp);
    s.push_str(&self.nonce.to_string());
    s.push_str(&self.previous_hash);
    self.hash = hash_string(&s);
  }

  pub fn mine_block(&mut self, difficulty: usize) {
    let hash_compare: String = String::from_utf8(vec![b'0'; difficulty]).unwrap();
    let mut hash_start: String = self.hash.chars().take(difficulty).collect();
    while !hash_compare.eq(&hash_start) {
      self.nonce += 1;
      self.calc_hash();
      hash_start = self.hash.chars().take(difficulty).collect();
    }
  }

  pub fn set_previous_hash(&mut self, hash: &str) {
    self.previous_hash = hash.to_string();
  }

  pub fn get_previous_hash(&self) -> &str {
    return &self.previous_hash[..];
  }

  pub fn get_hash(&self) -> &str {
    return &self.hash[..];
  }
}

impl Blockchain {
  pub fn new() -> Blockchain {
    return Blockchain {
      blocks: Vec::<Block>::new() 
    };
  }

  pub fn add_block(&mut self, block: Block) -> Result<&'static str, &'static str> {
      match self.get_last_hash() {
      Err(s) => {
        // empty chain, just add
        self.blocks.push(block);
        return Ok("Block added to the chain");
      },
      Ok(h) => {
        // chain not empty, check correct
        if h.eq(block.get_previous_hash()) {
          self.blocks.push(block);
          return Ok("Block added to the chain");
        }
      }
    }
    return Err("Invalid block");
  }

  pub fn len(&self) -> usize {
    return self.blocks.len();
  }

  pub fn get_last_hash(&self) -> Result<&str, &'static str> {
    match self.blocks.last() {
        None => Err("No blocks in the chain"),
        Some(b) => Ok(b.get_hash()) 
    }
  }
}

impl Printer for Block {
  fn print(&self) -> String {
    return format!(
      "{{Transactions: {}, Timestamp: {}, Nonce: {}, Hash: {}, PreviousHash: {}}}",
      self.get_transactions(), self.timestamp, self.nonce, self.hash, self.previous_hash);
  }
}

impl Printer for Transaction {
  fn print(&self) -> String {
    return format!("{{From: {}, To: {}, Ammount: {}, \
Key: {}, Timestamp: {}, Hash: {}, Signature: {}}}",
      self.from_address, self.to_address, self.ammount,
      self.public_key, self.timestamp, self.hash, self.signature);
  }
}

impl Printer for Blockchain {
  fn print(&self) -> String {
    let mut s: String = String::new();
    for block in self.blocks.iter() {
        if s.len() == 0 {
            s.push_str("[");
        } else {
            s.push_str(",");
        }
        s.push_str(block.print().as_str());
    }
    if s.len() > 0 {
        s.push_str("]");
    }
    return s;
  }
}


