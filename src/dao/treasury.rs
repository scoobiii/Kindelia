// Kindelia DAO Treasury Module
// -------------------------------------
// Responsável: Carla Mendes (DevOps)
// Tipo: DAO / Economia
// Status: Em Desenvolvimento
// -------------------------------------
//
// Descrição:
// Implementa o cofre descentralizado responsável pela gestão
// dos fundos da DAO, distribuição de tokens e execução de grants.

pub struct Treasury {
    pub total_funds: u128,
}

impl Treasury {
    pub fn new() -> Self {
        Treasury { total_funds: 0 }
    }

    pub fn deposit(&mut self, amount: u128) {
        self.total_funds += amount;
    }

    pub fn spend(&mut self, amount: u128) -> bool {
        if self.total_funds >= amount {
            self.total_funds -= amount;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Teste de integração do módulo Treasury
    // ---------------------------------------
    // Responsável: QA Team
    // Objetivo: Verificar depósitos e retiradas da DAO

    #[test]
    fn test_treasury_flow() {
        let mut t = Treasury::new();
        t.deposit(1000);
        assert_eq!(t.total_funds, 1000);
        assert!(t.spend(500));
        assert_eq!(t.total_funds, 500);
    }
}
