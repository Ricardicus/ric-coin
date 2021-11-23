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
  let mut blockchain: Blockchain = Blockchain::new();

  println!("I got this block:\n{}", block.print());
  println!("and I will now mine this block with difficulty {}..", difficulty);
  block.set_previous_hash(blockchain.get_last_hash().expect("No genesis block"));
  block.mine_block(difficulty);
  println!("Block mined:\n{}", block.print());
  block.print();

  blockchain.add_block(block).expect("Adding a block");
  println!("Blockchain with one block: {}", blockchain.print());

  let t3: Transaction = Transaction::new(
      &keys, "Rickard 1".to_string(), "Rickard 2".to_string(), 1337);
  let t4: Transaction = Transaction::new(
      &keys, "Rickard 2".to_string(), "Rickard 1".to_string(), 1337);
  
  let mut block2: Block = Block::new();
  block2.add_transaction(t3);
  block2.add_transaction(t4); 
  block2.set_previous_hash(blockchain.get_last_hash().unwrap());
  println!("I got this new block:\n{}", block2.print());
  println!("and I will now mine this block with difficulty {}..", difficulty);
  block2.mine_block(difficulty);
  println!("Block mined:\n{}", block2.print());

  blockchain.add_block(block2).expect("Adding a block");
  println!("Blockchain with two blocks: {}", blockchain.print());
  
  println!("Blockchain valid? {}", blockchain.verify());
}

