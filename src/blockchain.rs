use crate::block::Block;
use crate::transaction::Transaction;
use chrono::Utc;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        // Create the Genesis Block
        let genesis_block = Block::new(
            0,
            vec![], // Start with no transactions
            "0".to_string(),
        );

        let difficulty = 2;

        Blockchain {
            chain: vec![genesis_block],
            difficulty,
            mempool: vec![], // Start with an empty mempool
        }
    }

    // Adds a new, verified transaction to the mempool
    pub fn add_transaction_to_mempool(&mut self, transaction: Transaction) -> bool {
        // 1. Verify the transaction's signature
        if !transaction.verify_signature() {
            println!("Invalid transaction signature. Rejecting.");
            return false;
        }

        // 2. If valid, add it to the mempool
        self.mempool.push(transaction);
        true
    }

    // A helper function to get the most recent block
    pub fn get_last_block(&self) -> &Block {
        // This is safe because `new()` guarantees at least one block
        self.chain.last().unwrap()
    }

    // This is the updated "mining" function
    pub fn mine_block(&mut self) {
        // We will reward the miner in Phase 4.
        // For now, we just take all transactions from the mempool.
        // .clone() is a simple way to do this; .drain() is more efficient.
        let transactions = self.mempool.clone();

        let previous_hash = self.get_last_block().hash.clone();
        let mut index = self.chain.len() as u64;
        let mut nonce = 0;
        let timestamp = Utc::now().timestamp();

        // Start the mining loop
        loop {
            // Create a temporary block with the current nonce and transactions
            let mut block = Block {
                index,
                timestamp,
                transactions: transactions.clone(), // Use the mempool transactions
                previous_hash: previous_hash.clone(),
                hash: String::new(),
                nonce,
            };

            // Calculate the hash for this temporary block
            let hash = block.calculate_hash();

            // ---- THIS IS THE PROOF-OF-WORK CHECK ----
            let prefix = "0".repeat(self.difficulty);

            if hash.starts_with(&prefix) {
                // If it does, we found a valid block!
                println!("Block Mined! Hash: {}", hash);
                block.hash = hash; // Set the valid hash
                self.chain.push(block); // Add the block to the chain

                // Clear the mempool since these transactions are now in a block
                self.mempool.clear();
                break; // Exit the loop
            } else {
                // If it doesn't, increment the nonce and try again
                nonce += 1;
                // Update index in case chain length changed (for future P2P)
                index = self.chain.len() as u64;
            }
        }
    }

    // This function validates the entire chain
    pub fn is_chain_valid(&self) -> bool {
        // Start at block 1 (index 1), not the genesis block
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // 1. Check if the current block's hash is valid
            if current_block.hash != current_block.calculate_hash() {
                println!("Validation Failed: Block {} hash is invalid.", current_block.index);
                return false;
            }

            // 2. Check if the previous_hash field links correctly
            if current_block.previous_hash != previous_block.hash {
                println!("Validation Failed: Block {} previous_hash does not match block {}.", current_block.index, previous_block.index);
                return false;
            }

            // 3. Check if the block's hash meets the PoW difficulty
            let prefix = "0".repeat(self.difficulty);
            if !current_block.hash.starts_with(&prefix) {
                println!("Validation Failed: Block {} hash does not meet difficulty.", current_block.index);
                return false;
            }
        }

        // If we checked all blocks and all are valid
        true
    }
}