//! src/wallet/keys.rs
//! Simple key management (mocked, deterministic).
//! Responsibility:
//! - generate deterministic keypair from seed
//! - sign/verify minimal API used by wallet

use sha2::{Digest, Sha256};

/// A tiny mock Keypair used for signing in tests / MVP.
/// NOTE: this is NOT production-grade crypto — swap for real Ed25519 lib for production.
#[derive(Debug, Clone)]
pub struct Keypair {
    pub seed: String,
    pub pubkey: String,
    pub privkey: String,
}

impl Keypair {
    pub fn from_seed(seed: &str) -> Self {
        // derive deterministic "keys" via SHA256 (mock)
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let priv_raw = hasher.finalize();
        let privhex = hex::encode(&priv_raw);
        // pubkey = sha256(privkey)
        let mut hasher2 = Sha256::new();
        hasher2.update(privhex.as_bytes());
        let pubraw = hasher2.finalize();
        let pubhex = hex::encode(&pubraw);
        Keypair {
            seed: seed.to_string(),
            pubkey: pubhex,
            privkey: privhex,
        }
    }

    pub fn sign(&self, data: &[u8]) -> String {
        // mock signature = sha256(privkey || data)
        let mut hasher = Sha256::new();
        hasher.update(self.privkey.as_bytes());
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    pub fn public_key_short(&self) -> String {
        // concise id
        self.pubkey.chars().take(12).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_keys() {
        let a = Keypair::from_seed("seed-x");
        let b = Keypair::from_seed("seed-x");
        assert_eq!(a.pubkey, b.pubkey);
        assert_eq!(a.privkey, b.privkey);
    }

    #[test]
    fn sign_differs() {
        let k = Keypair::from_seed("s");
        let s1 = k.sign(b"hello");
        let s2 = k.sign(b"bye");
        assert_ne!(s1, s2);
    }
}
