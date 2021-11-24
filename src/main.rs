mod keygenerator;
use keygenerator::{KeyMaster};
mod blockchain;
use blockchain::{Blockchain, Printer, Transaction};

fn main() {
  let keys = KeyMaster::new();

  let keys_another_address = KeyMaster::new();
  let mut blockchain: Blockchain = Blockchain::new();
    
  let t1: Transaction = Transaction::new(
      &keys, keys_another_address.public_key.clone(), 1337);
  let t2: Transaction = Transaction::new(
      &keys, keys_another_address.public_key.clone(), 1337);
  
  blockchain.add_transaction(t1).expect("Adding a transaction");
  blockchain.add_transaction(t2).expect("Adding a transaction");

  println!("Mining a block...");
  blockchain.mine_pending_transactions(&keys, keys.public_key.clone());
  println!("Blockchain after mining a block: {}", blockchain.print());

  let t3: Transaction = Transaction::new(
      &keys, keys_another_address.public_key.clone(), 1337);
  let t4: Transaction = Transaction::new(
      &keys, keys_another_address.public_key.clone(), 1337);
  
  blockchain.add_transaction(t3).expect("Adding a transaction");
  blockchain.add_transaction(t4).expect("Adding a transaction");

  println!("Mining another block...");
  blockchain.mine_pending_transactions(&keys, keys.public_key.clone());

  println!("Blockchain with two blocks mined: {}", blockchain.print()); 
  println!("Blockchain valid? {}", blockchain.verify());
}

