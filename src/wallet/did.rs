//! src/wallet/did.rs
//! Simple DID (Decentralized ID) helper.
//! Responsibilities:
//! - Create a human-friendly DID
//! - Resolve identity fields (mock) used in proposals and DAO interactions

use crate::wallet::keys::Keypair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DID {
    pub id: String,
    pub handle: String,
    pub public_key: String,
}

impl DID {
    /// Create DID from keypair (mock)
    pub fn from_keypair(k: &Keypair, handle: Option<&str>) -> Self {
        let id = format!("did:kind:{}", k.public_key_short());
        let hn = handle.unwrap_or("anon").to_string();
        DID {
            id,
            handle: hn,
            public_key: k.pubkey.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::keys::Keypair;
    #[test]
    fn did_from_keypair() {
        let k = Keypair::from_seed("seed-did");
        let d = DID::from_keypair(&k, Some("zeh"));
        assert!(d.id.starts_with("did:kind:"));
        assert_eq!(d.handle, "zeh");
    }
}
