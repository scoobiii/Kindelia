// src/api/hvm_integration.rs
use crate::hvm::HvmRuntime;
use crate::dao::tokenomics_integration::*;
use crate::wallet::storage::*;

pub struct EnergyApi {
    hvm: HvmRuntime,
}

impl EnergyApi {
    pub fn new(hvm: HvmRuntime) -> Self {
        Self { hvm }
    }

    // Recebe dados de smart meters/inversores
    pub fn submit_reading(&self, device_id: &str, reading: f64) -> Result<(), String> {
        // Validação de assinatura digital (simulado)
        if reading < 0.0 { return Err("Leitura inválida".into()); }
        self.hvm.record_reading(device_id, reading)?;
        Ok(())
    }

    // Tokeniza energia ou ativos
    pub fn mint_tokens(&self, user: &str, amount: f64, asset_type: &str) -> Result<String, String> {
        tokenomics_mint(user, amount, asset_type)
    }

    // Transferência de tokens para financiadores
    pub fn transfer_tokens(&self, from: &str, to: &str, amount: f64) -> Result<String, String> {
        tokenomics_transfer(from, to, amount)
    }
}
