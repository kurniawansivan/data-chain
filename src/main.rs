// 1. Include all our modules
mod block;
mod blockchain;
mod transaction;
mod wallet;

// 2. Bring our key structs into scope
use blockchain::Blockchain;
use transaction::{Transaction, TransactionPayload};
use wallet::Wallet;

fn main() {
    // --- SETUP ---
    // 1. Create a new blockchain
    let mut chain = Blockchain::new();
    println!("Blockchain created!");

    // 2. Create two wallets
    let alice_wallet = Wallet::new();
    let miner_wallet = Wallet::new(); // The miner needs an address too

    println!("Created Alice's wallet.");
    println!("Created Miner's wallet.");

    // --- CREATE TRANSACTION ---
    // 3. Alice creates a transaction to pay the miner 10 "DataCoin"
    //    (Even though balances aren't enforced yet, we can create the transaction)
    let tx_payload = TransactionPayload::Transfer {
        recipient: miner_wallet.verifying_key, // Miner's public key
        amount: 10,
    };
    
    let tx1 = Transaction::new(&alice_wallet, tx_payload);
    println!("Alice created transaction: {:?}", tx1.payload);

    // --- MEMPOOL ---
    // 4. Add the transaction to the blockchain's mempool
    let added = chain.add_transaction_to_mempool(tx1);
    if added {
        println!("Transaction added to mempool.");
    } else {
        println!("Failed to add transaction.");
    }

    // --- MINING ---
    // 5. Mine a new block.
    //    This should grab all transactions from the mempool.
    println!("Mining block 1...");
    chain.mine_block(); // Notice: no arguments needed!

    // --- VERIFICATION ---
    // 6. Print and validate the chain
    println!("\nFull Blockchain:\n{:#?}", chain);
    
    let is_valid = chain.is_chain_valid();
    println!("\nIs the blockchain valid? {}", is_valid);

    // 7. Check the block
    if let Some(block1) = chain.chain.get(1) {
        println!("\nTransactions in Block 1:");
        println!("{:#?}", block1.transactions);
    }
}