mod keygenerator;
use keygenerator::{KeyMaster};
mod blockchain;
use blockchain::{Block, Printer, Transaction};

fn main() {
  let keys = KeyMaster::new();
  println!("secret: {}, public: {}", keys.secret_key, keys.public_key);
  let t1: Transaction = Transaction::new(
      &keys, "Rickard 1".to_string(), "Rickard 2".to_string(), 1337);
  let t2: Transaction = Transaction::new(
      &keys, "Rickard 2".to_string(), "Rickard 1".to_string(), 1337);
  
  let mut block: Block = Block::new();
  block.add_transaction(t1);
  block.add_transaction(t2);
  let difficulty: usize = 4;
  println!("I got this block:\n{}", block.print());
  println!("and I will now mine this block with difficulty {}..", difficulty);
  block.mine_block(difficulty);
  println!("Block mined:\n{}", block.print());
  block.print();
}

