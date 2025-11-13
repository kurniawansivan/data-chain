// 1. We import `Signer` which will be enabled by our feature flag
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng; // OsRng is a secure random number generator

// We derive Debug so we can print the wallet (for testing)
#[derive(Debug)]
pub struct Wallet {
    // The signing key is the "private key". It MUST be kept secret.
    pub signing_key: SigningKey,
    // The verifying key is the "public key". This is the "address".
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    // Creates a new wallet with a fresh keypair
    pub fn new() -> Self {
        let mut csprng = OsRng; // Create a cryptographically secure random generator
        
        // 2. This `generate` function is what we are enabling
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Wallet {
            signing_key,
            verifying_key,
        }
    }
}