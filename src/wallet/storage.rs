//! src/wallet/storage.rs
//! Local secure-ish storage for wallet state (mocked).
//! Responsibilities:
//! - store pending txs, sent txs
//! - store simple settings (large-font true/false, tts enabled)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct LocalStorage {
    id: String,
    inner: Arc<Mutex<HashMap<String, String>>>,
}

impl LocalStorage {
    pub fn open(id: &str) -> Self {
        LocalStorage {
            id: id.to_string(),
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn save_pending(&self, payload: &str) {
        let mut m = self.inner.lock().unwrap();
        m.insert(format!("pending:{}", payload), "1".to_string());
    }

    pub fn mark_sent(&self, txid: &str) {
        let mut m = self.inner.lock().unwrap();
        m.insert(format!("sent:{}", txid), "1".to_string());
    }

    pub fn set_setting(&self, key: &str, value: &str) {
        let mut m = self.inner.lock().unwrap();
        m.insert(format!("setting:{}", key), value.to_string());
    }

    pub fn get_setting(&self, key: &str) -> Option<String> {
        let m = self.inner.lock().unwrap();
        m.get(&format!("setting:{}", key)).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn storage_basic() {
        let s = LocalStorage::open("idxx");
        s.save_pending("pay:1");
        s.mark_sent("tx1");
        s.set_setting("big_font", "true");
        assert_eq!(s.get_setting("big_font").unwrap(), "true");
    }
}
