use crate::block::Block;

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

    // 6. A *temporary* function to add a new block (for testing)
    //    We will replace this in Phase 2 with `mine_block()`.
    pub fn add_block(&mut self, transactions: Vec<String>) {
        // Get the hash of the last block in the chain
        let previous_hash = self.get_last_block().hash.clone();
        
        // Create the new block
        let new_block = Block::new(
            self.chain.len() as u64, // index is the current length of the chain
            transactions,
            previous_hash,
        );

        // Add the new block to the chain
        self.chain.push(new_block);
    }
}