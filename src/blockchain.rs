use crate::block::Block;
use chrono::Utc;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize, // This will be used in Phase 2
}

impl Blockchain {
    // 1. The constructor for a new blockchain
    pub fn new() -> Self {
        // 2. Create the Genesis Block
        let genesis_block = Block::new(
            0,                          // index
            vec!["Genesis Block".to_string()], // transactions (just a placeholder)
            "0".to_string(),            // previous_hash (it has no predecessor)
        );

        // 3. Set the difficulty for Phase 2
        let difficulty = 2; // e.g., hashes must start with "00"

        // 4. Return a new Blockchain instance
        Blockchain {
            chain: vec![genesis_block], // The chain starts with the genesis block
            difficulty,
        }
    }

    // 5. A helper function to get the most recent block
    pub fn get_last_block(&self) -> &Block {
        // .last() returns an Option, so we .unwrap() it.
        // This is safe because our `new()` function guarantees
        // there is always at least one block (the genesis block).
        self.chain.last().unwrap()
    }

    // This is the new "mining" function that replaces `add_block`
    pub fn mine_block(&mut self, transactions: Vec<String>) {
        let previous_hash = self.get_last_block().hash.clone();
        let mut index = self.chain.len() as u64;
        let mut nonce = 0;
        let timestamp = Utc::now().timestamp();

        // Start the mining loop
        loop {
            // Create a temporary block with the current nonce
            let mut block = Block {
                index,
                timestamp,
                transactions: transactions.clone(),
                previous_hash: previous_hash.clone(),
                hash: String::new(), // Hash will be calculated
                nonce,
            };

            // Calculate the hash for this temporary block
            let hash = block.calculate_hash();

            // ---- THIS IS THE PROOF-OF-WORK CHECK ----
            // Create the required prefix (e.g., "00" if difficulty is 2)
            let prefix = "0".repeat(self.difficulty);

            // Check if the hash has the required prefix
            if hash.starts_with(&prefix) {
                // If it does, we found a valid block!
                println!("Block Mined! Hash: {}", hash);
                block.hash = hash; // Set the valid hash
                self.chain.push(block); // Add the block to the chain
                break; // Exit the loop
            } else {
                // If it doesn't, increment the nonce and try again
                nonce += 1;
                // We'll also update the index in case another node found a block
                // (This is a simplified view for now but good practice)
                index = self.chain.len() as u64;
            }
        }
    }
}