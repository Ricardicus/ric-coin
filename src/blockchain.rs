use crate::keygenerator::{hash_string, KeyMaster};
use chrono;

#[derive(Clone)]
pub struct Transaction {
  to_address: String,
  ammount: u64,
  timestamp: String,
  hash: String,
  signature: String,
  public_key: String
}

#[derive(Clone)]
pub struct Block {
  timestamp: String,
  nonce: u64,
  hash: String,
  previous_hash: String,
  transactions: Vec<Transaction>
}

pub struct Blockchain {
  blocks: Vec<Block>,
  mining_reward: u64,
  difficulty: usize,
  pending_transactions: Vec<Transaction>
}

pub trait Printer {
  fn print(&self) -> String;
}

impl Transaction {
  pub fn new(
      keys: &KeyMaster,
      to_address: String,
      ammount: u64) -> Transaction {
    let mut t: Transaction = Transaction {
      timestamp : format!("{:?}", chrono::offset::Utc::now()),
      ammount : ammount,
      to_address : to_address,
      hash : String::new(),
      signature: String::new(),
      public_key: keys.public_key.to_string()
    };
    t.calc_hash();
    t.signature = keys.sign(t.hash.to_string());
    return t;
  }

  pub fn calc_hash(&mut self) {
    let mut s: String = String::new();
    s.push_str(&self.to_address);
    s.push_str(&self.public_key);
    s.push_str(&self.ammount.to_string());
    s.push_str(&self.timestamp);
    self.hash = hash_string(&s);
}

  pub fn get_hash(&self) -> &str {
    return &self.hash[..];
  }

  pub fn verify(&self) -> bool {
    let message: String = self.hash.to_string();  
    if self.ammount == 0 {
      return false;
    }
    let keys = KeyMaster::new();

    if keys.verify_with_public_key(self.public_key.clone(), message.clone(), self.signature.clone()) {
      return true;
    }
    return false;
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

  pub fn calc_hash(&mut self) -> String {
    let mut s: String = String::new();
    s.push_str(&self.get_transactions());
    s.push_str(&self.timestamp);
    s.push_str(&self.nonce.to_string());
    s.push_str(&self.previous_hash);
    return hash_string(&s);
  }

  pub fn mine_block(&mut self, difficulty: usize) {
    let hash_compare: String = String::from_utf8(vec![b'0'; difficulty]).unwrap();
    let mut hash_start: String = self.hash.chars().take(difficulty).collect();
    while !hash_compare.eq(&hash_start) {
      self.nonce += 1;
      self.hash = self.calc_hash();
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
 
  pub fn verify(&self) -> bool {
    for transaction in self.transactions.iter() {
      if ! transaction.verify() {
        return false;
      }
    }
    return true;
  }
}

impl Blockchain {
  pub fn new() -> Blockchain {
    let mut blockchain: Blockchain = Blockchain {
      blocks: Vec::<Block>::new(),
      mining_reward: 10,
      difficulty: 4,
      pending_transactions: Vec::<Transaction>::new()
    };
    let genesis: Block = blockchain.create_genesis_block();
    blockchain.blocks.clear();
    blockchain.blocks.push(genesis);
    return blockchain;
  }

  pub fn create_genesis_block(&self) -> Block {
    let mut genesis_block: Block = Block::new();
    genesis_block.timestamp = "Beginning of time".to_string();
    genesis_block.hash = genesis_block.calc_hash();
    return genesis_block;
  }

  pub fn add_block(&mut self, block: Block) -> Result<&'static str, &'static str> {
    match self.get_last_hash() {
      Err(_) => {
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

  pub fn get_last_hash(&self) -> Result<&str, &'static str> {
    match self.blocks.last() {
        None => Err("No blocks in the chain"),
        Some(b) => Ok(b.get_hash()) 
    }
  }

  pub fn verify(&self) -> bool {
    let genesis = self.create_genesis_block();
    let first: Block = self.blocks[0].clone();

    if genesis != first {
      return false;
    }

    let mut previous_block: Option<&Block> = None;
    for block in self.blocks.iter().skip(1) {
      if ! block.verify() {
        return false;
      }
      match previous_block {
        None => (),
        Some(b) => {
          if !block.previous_hash.eq(&b.hash[..]) {
            println!("prev_block hash {}, block previous_hash {}", &block.previous_hash[..], &b.hash[..]);
            return false;
          }
        }
      }
      previous_block = Some(block);
    }
    return true;
  }

  pub fn add_transaction(&mut self, transaction: Transaction) -> Result<&'static str, &'static str> {
    if !transaction.verify() {
      return Err("Invalid transaction");
    }

    match self.get_balance(&transaction.public_key[..]) {
      Ok(balance) => {
        if balance < transaction.ammount {
          return Err("Insufficient funds."); 
        }
      },
      Err(s) => {
        return Err(s);
      }
    }

    self.pending_transactions.push(transaction);
    return Ok("Transaction added");
  }

  pub fn mine_pending_transactions(&mut self, keys: &KeyMaster, reward_address: String) {
    let reward: Transaction = Transaction::new(
        keys, reward_address.clone(), self.mining_reward);
    let mut block = Block::new();
    block.add_transaction(reward);
    for transaction in self.pending_transactions.iter() {
      block.add_transaction(transaction.clone());
    }
    block.set_previous_hash(self.get_last_hash().expect("No genesis block"));
    block.mine_block(self.difficulty);
    self.blocks.push(block);
    self.pending_transactions.clear();
  }

  pub fn get_balance(&self, address: &str) -> Result<u64, &'static str> {
    let mut balance: u64 = 0;
    for block in self.blocks.iter().skip(1) {
      for transaction in block.transactions.iter() {
        if transaction.to_address.eq(address) {
          balance += transaction.ammount;
        }
        // public key is 'from_address'
        else if transaction.public_key.eq(address) {
          if transaction.ammount > balance {
            return Err("Invalid chain");
          }
          balance -= transaction.ammount;
        }
      }
    }
    return Ok(balance);
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
Timestamp: {}, Hash: {}, Signature: {}}}",
      self.public_key, self.to_address, self.ammount,
      self.timestamp, self.hash, self.signature);
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

impl PartialEq<Block> for Block {
  fn eq(&self, other: &Block) -> bool {
    if self.timestamp[..].eq(&other.timestamp[..]) &&
      self.hash[..].eq(&other.hash[..]) &&
      self.previous_hash[..].eq(&other.previous_hash[..]) &&
      self.get_transactions().eq(&other.get_transactions()) {
      return true;
    }
    return false;
  }
}

