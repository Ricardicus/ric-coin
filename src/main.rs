mod keygenerator;
use keygenerator::KeyMaster;
mod blockchain;
use blockchain::{Blockchain, Printer, Transaction};

fn main() {
    // Some addresses
    let keys_miner = KeyMaster::new();
    let keys_miner_reward = KeyMaster::new();
    let keys_some_address = KeyMaster::new();
    let keys_another_address = KeyMaster::new();

    let mut blockchain: Blockchain = Blockchain::new();

    // Mine first block
    println!("Mining the first block");
    blockchain.mine_pending_transactions(&keys_miner, keys_miner_reward.public_key.clone());

    let t1: Transaction =
        Transaction::new(&keys_miner_reward, keys_some_address.public_key.clone(), 2);
    let t2: Transaction = Transaction::new(
        &keys_some_address,
        keys_another_address.public_key.clone(),
        1,
    );

    blockchain
        .add_transaction(t1)
        .expect("Adding a transaction");
    println!("Mining another block...");
    blockchain.mine_pending_transactions(&keys_miner, keys_miner_reward.public_key.clone());

    blockchain
        .add_transaction(t2)
        .expect("Adding a transaction");
    println!("Mining another block...");
    blockchain.mine_pending_transactions(&keys_miner, keys_miner_reward.public_key.clone());

    let t3: Transaction = Transaction::new(
        &keys_miner_reward,
        keys_another_address.public_key.clone(),
        3,
    );
    let t4: Transaction = Transaction::new(
        &keys_another_address,
        keys_some_address.public_key.clone(),
        1,
    );

    blockchain
        .add_transaction(t3)
        .expect("Adding a transaction");
    blockchain
        .add_transaction(t4)
        .expect("Adding a transaction");

    println!("Mining another block...");
    blockchain.mine_pending_transactions(&keys_miner, keys_miner_reward.public_key.clone());

    println!(
        "Miner reward wallet ammount {}",
        blockchain
            .get_balance(&keys_miner_reward.public_key[..])
            .unwrap()
    );
    println!(
        "Some wallet ammount {}",
        blockchain
            .get_balance(&keys_some_address.public_key[..])
            .unwrap()
    );
    println!(
        "Another wallet ammount {}",
        blockchain
            .get_balance(&keys_another_address.public_key[..])
            .unwrap()
    );

    println!("Blockchain valid? {}", blockchain.verify());
}
