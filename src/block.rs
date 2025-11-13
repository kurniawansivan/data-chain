use crate::transaction::Transaction;
use serde::Serialize;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde_json;

#[derive(Serialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, 
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // This is for Proof-of-Work in Phase 2
}

impl Block {
    // 1. The "new" function (constructor)
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(), // Start with an empty hash
            nonce: 0,
        };

        // Calculate the hash for this new block
        block.hash = block.calculate_hash();
        block
    }

    // 2. The hash calculation function
    pub fn calculate_hash(&self) -> String {
        // Create a copy of the block data for serialization
        // We can't serialize the 'hash' field itself
        let mut block_data = self.clone();
        block_data.hash = String::new(); // Ensure hash is empty when hashing

        // Serialize the block_data into a JSON string
        // .unwrap() is used for simplicity; in real code, we'd handle errors
        let serialized_data = serde_json::to_string(&block_data).unwrap();

        // Create a new SHA-256 hasher
        let mut hasher = Sha256::new();

        // Write the serialized data as bytes into the hasher
        hasher.update(serialized_data.as_bytes());

        // Get the final hash result
        let result = hasher.finalize();

        // Format the result as a hexadecimal string
        format!("{:x}", result)
    }
}