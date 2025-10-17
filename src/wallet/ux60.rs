//! src/wallet/ux60.rs
//! Accessibility helpers and "Modo Avó" one-click flows.
//! Responsibilities:
//! - provide simplified UI state and confirm flows used by frontend
//! - provide helpers for large fonts, high contrast and simplified labels

use crate::wallet::storage::LocalStorage;
use crate::wallet::keys::Keypair;
use crate::wallet::did::DID;

/// GrandmaMode is a helper that encapsulates the simplified UX.
/// It does not render UI; it provides a deterministic state for rendering.
pub struct GrandmaMode<'a> {
    wallet_id: String,
    storage: &'a LocalStorage,
    default_font: u8, // percentage of base font (e.g., 150)
}

impl<'a> GrandmaMode<'a> {
    pub fn new(wallet: &'a mut crate::Wallet) -> Self {
        // ensure default settings saved
        wallet.storage.set_setting("big_font", "true");
        wallet.storage.set_setting("contrast", "high");
        GrandmaMode {
            wallet_id: wallet.id.clone(),
            storage: &wallet.storage,
            default_font: 160,
        }
    }

    /// Return UI hints (labels, font size, contrast)
    pub fn ui_hints(&self) -> (String, u8, String) {
        ("Modo Avó: Transações em 1 clique".to_string(), self.default_font, "high".to_string())
    }

    /// One-click send flow: given parsed intent, perform send. Returns friendly message.
    pub fn one_click_send(&mut self, wallet: &mut crate::Wallet, to: &str, amount: u64) -> Result<String,String> {
        // confirm minimal checks
        if amount == 0 {
            return Err("Valor inválido".to_string());
        }
        // perform send (wallet.signs + posts)
        match wallet.send_simple(to, amount) {
            Ok(txid) => Ok(format!("Transação enviada ✅\nID: {}", txid)),
            Err(e) => Err(format!("Erro ao enviar: {}", e)),
        }
    }

    /// Generate a DID summary for the UI
    pub fn did_summary(&self, k: &Keypair) -> DID {
        DID::from_keypair(k, Some("avó"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::keys::Keypair;
    use crate::wallet::storage::LocalStorage;
    use crate::Wallet;

    #[test]
    fn granny_ui_hints() {
        let mut w = Wallet::new("seed-grandma");
        let mut gm = w.grandma_mode();
        let hints = gm.ui_hints();
        assert!(hints.0.contains("Modo Avó"));
        assert!(hints.1 >= 120);
    }

    #[test]
    fn granny_one_click() {
        let mut w = Wallet::new("seed-grandma-2");
        let mut gm = w.grandma_mode();
        let res = gm.one_click_send(&mut w, "friend", 7);
        assert!(res.is_ok());
    }
}
