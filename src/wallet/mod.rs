//! src/wallet/mod.rs
//! Wallet module — public facade for wallet functionality.
//! Responsibilities:
//! - expose keys, did, storage, UX & voice features
//! - provide simple high-level API for apps (one-click send/send_confirm etc)

pub mod keys;
pub mod did;
pub mod storage;
pub mod voice_ui;
pub mod ux60;

use crate::api::client;
use keys::Keypair;
use storage::LocalStorage;
use ux60::GrandmaMode;

#[derive(Debug)]
pub struct Wallet {
    pub id: String,
    pub keys: Keypair,
    pub storage: LocalStorage,
}

impl Wallet {
    /// Create a new wallet with a deterministic seed phrase (for demo/test).
    pub fn new(seed: &str) -> Self {
        let keys = Keypair::from_seed(seed);
        let id = keys.public_key_short();
        let storage = LocalStorage::open(&id);
        Wallet { id, keys, storage }
    }

    /// One-click send used by Grandma Mode: simplified UX.
    /// This is synchronous and returns a mock tx id on success.
    pub fn send_simple(&mut self, to: &str, amount: u64) -> Result<String, String> {
        // UX layer should have confirmed already; just sign and post.
        let payload = format!("send:{}:{}", to, amount);
        let sig = self.keys.sign(payload.as_bytes());
        // store pending tx in local storage
        self.storage.save_pending(&payload);
        // simulated post via API client (mockable)
        match client::post_transaction_mock(&payload, &sig) {
            Ok(txid) => {
                self.storage.mark_sent(&txid);
                Ok(txid)
            }
            Err(e) => Err(e),
        }
    }

    /// Expose grandma mode helper
    pub fn grandma_mode(&mut self) -> GrandmaMode {
        GrandmaMode::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_create_and_send_simple() {
        let mut w = Wallet::new("test-seed-1");
        let res = w.send_simple("recipient-addr", 10);
        assert!(res.is_ok());
        let txid = res.unwrap();
        assert!(txid.len() > 0);
    }
}
