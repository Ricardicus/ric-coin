mod keygenerator;
use keygenerator::{KeyMaster};
mod blockchain;
use blockchain::{Blockchain, Printer, Transaction};

fn main() {
  let keys = KeyMaster::new();
  println!("keys:\n  secret: {}, public: {}", keys.secret_key, keys.public_key);
  let mut blockchain: Blockchain = Blockchain::new(keys);
    
  let t1: Transaction = Transaction::new(
      blockchain.get_keys(), "Rickard 1".to_string(), "Rickard 2".to_string(), 1337);
  let t2: Transaction = Transaction::new(
      blockchain.get_keys(), "Rickard 2".to_string(), "Rickard 1".to_string(), 1337);
  
  blockchain.add_transaction(t1);
  blockchain.add_transaction(t2);

  blockchain.mine_pending_transactions("Rickard".to_string());
  println!("Blockchain after mining a block: {}", blockchain.print());

  let t3: Transaction = Transaction::new(
      blockchain.get_keys(), "Rickard 1".to_string(), "Rickard 2".to_string(), 1337);
  let t4: Transaction = Transaction::new(
      blockchain.get_keys(), "Rickard 2".to_string(), "Rickard 1".to_string(), 1337);
  
  blockchain.add_transaction(t3);
  blockchain.add_transaction(t4);

  blockchain.mine_pending_transactions("Rickard".to_string());

  println!("Blockchain with two blocks mined: {}", blockchain.print());
  
  println!("Blockchain valid? {}", blockchain.verify());
}

