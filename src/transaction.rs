use serde::Serialize;
use crate::wallet::Wallet;
// Add `Verifier` here
use ed25519_dalek::{Signature, VerifyingKey, Signer, Verifier}; 
use chrono::prelude::*;
use serde_json; // We need this for serialization

// This enum represents the *content* or *purpose* of a transaction
#[derive(Serialize, Debug, Clone)]
pub enum TransactionPayload {
    // 1. Transfer our native "DataCoin"
    Transfer {
        recipient: VerifyingKey, // The public key of the receiver
        amount: u64,
    },
    // 2. List a new piece of data for sale
    ListData {
        data_hash: String, // The IPFS hash of the data file
        price: u64,        // How many "DataCoin" it costs
    },
    // 3. Purchase a piece of data
    PurchaseData {
        data_id: String, // A unique ID for the data listing
    },
}

// This struct is the final, signed "envelope"
// that goes onto the blockchain
#[derive(Serialize, Debug, Clone)]
pub struct Transaction {
    pub sender: VerifyingKey, // Public key of the person sending
    pub timestamp: i64,       // Time the transaction was created
    pub payload: TransactionPayload, // The actual action (Transfer, List, etc.)
    pub signature: Signature, // The cryptographic signature
}

// This block holds the functions (methods) for the Transaction struct
impl Transaction {
    // This is our new constructor
    pub fn new(wallet: &Wallet, payload: TransactionPayload) -> Self {
        let timestamp = Utc::now().timestamp();

        // We need to serialize the payload to sign it
        // (Signing happens on the *bytes* of the data)
        let payload_bytes = serde_json::to_vec(&payload).unwrap();

        // Use the wallet's private key to sign the payload bytes
        let signature = wallet.signing_key.sign(&payload_bytes);

        // Return the new, completed Transaction
        Self {
            sender: wallet.verifying_key, // The sender is the wallet's public key
            timestamp,
            payload,
            signature,
        }
    }

    // This function verifies the signature of a transaction
    pub fn verify_signature(&self) -> bool {
        // Serialize the payload, just like we did when signing
        let payload_bytes = serde_json::to_vec(&self.payload).unwrap();

        // Use the sender's *public key* to verify the signature
        // This checks: "Did the owner of this public key *really* sign this payload?"
        // We use `verify` which is a method on the `Verifier` trait
        self.sender.verify(&payload_bytes, &self.signature).is_ok()
    }
}