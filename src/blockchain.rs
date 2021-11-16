use crate::hash_string;
use chrono;

pub struct Transaction {
  from_address: String,
  to_address: String,
  ammount: u64,
  timestamp: String
}

pub struct Block {
  timestamp: String,
  ammount: u64,
  nonce: u64,
  hash: String,
  previousHash: String,
  transactions: Vec<Transaction>
}

pub trait Printer {
  fn print(&self) -> String;
}

impl Block {
  pub fn new(
      transactions: Vec<Transaction>,
      ammount: u64) -> Block {
    let mut b: Block = Block {
      timestamp : format!("{:?}", chrono::offset::Utc::now()),
      ammount : ammount,
      nonce : 0,
      hash: String::new(),
      previousHash: String::new(),
      transactions: transactions
    };
    b.calcHash();
    return b;
  }

  pub fn getTransactions(&self) -> String {
    let mut s: String = String::new();
    s.push_str("[");
    for transaction in self.transactions.iter() {
      let t: String = transaction.print();
      if ( s.len() > 1 ) {
        s.push_str(", ");
      }
      s.push_str(&t);
    }
    s.push_str("]");
    return s;
  }

  pub fn calcHash(&mut self) {
    let mut s: String = String::new();
    s.push_str(&self.getTransactions());
    s.push_str(&self.timestamp);
    s.push_str(&self.nonce.to_string());
    s.push_str(&self.ammount.to_string());
    s.push_str(&self.previousHash);
    self.hash = hash_string(&s);
  }

  pub fn mineBlock(&mut self, difficulty: usize) {
    let hashCompare: String = String::from_utf8(vec![b'0'; difficulty]).unwrap();
    let mut hashStart: String = self.hash.chars().take(difficulty).collect();
    while (!hashCompare.eq(&hashStart)) {
      self.nonce += 1;
      self.calcHash();
      hashStart = self.hash.chars().take(difficulty).collect();
    }
  }
}

impl Printer for Block {
  fn print(&self) -> String {
    return format!(
      "[Transactions: {}, Timestamp: {}, Nonce: {}, Hash: {}, PreviousHash: {}]",
      self.getTransactions(), self.timestamp, self.nonce, self.hash, self.previousHash);
  }
}

impl Printer for Transaction {
  fn print(&self) -> String {
    return format!("[From: {}, To: {}, Ammount: {}, Timestamp: {}]",
      self.from_address, self.to_address, self.ammount,
      self.timestamp);
  }
}


