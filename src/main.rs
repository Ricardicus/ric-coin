mod keygenerator;
use keygenerator::{KeyMaster};
mod blockchain;
use blockchain::{Blockchain, Block, Printer, Transaction};

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

  let mut blockchain: Blockchain = Blockchain::new();
  blockchain.add_block(block);
  println!("Blockchain with one block: {}", blockchain.print());
  
  let mut t3: Transaction = Transaction::new(
      &keys, "Rickard 1".to_string(), "Rickard 2".to_string(), 1337);
  let mut t4: Transaction = Transaction::new(
      &keys, "Rickard 2".to_string(), "Rickard 1".to_string(), 1337);
  
  block = Block::new();
  block.add_transaction(t3);
  block.add_transaction(t4);
  println!("I got this new block:\n{}", block.print());
  println!("and I will now mine this block with difficulty {}..", difficulty);
  block.mine_block(difficulty);
  println!("Block mined:\n{}", block.print());
  block.print();

  blockchain.add_block(block);
  println!("Blockchain with two blocks: {}", blockchain.print());
  


}

