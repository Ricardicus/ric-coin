mod keygenerator;
use keygenerator::hash_string;
mod blockchain;
use blockchain::{Block, Printer, Transaction};

fn main() {
  let t1: Transaction = Transaction::new(
      "Rickard 1".to_string(), "Rickard 2".to_string(), 1337);
  let mut transactions: Vec<Transaction> = Vec::new();
  transactions.push(t1);
  let mut block: Block = Block::new(
      transactions, 1337
  );
  let difficulty: usize = 4;
  println!("I got this block:\n{}", block.print());
  println!("and I will now mine this block with difficulty {}..", difficulty);
  block.mine_block(difficulty);
  println!("Block mined:\n{}", block.print());
  block.print();
}
