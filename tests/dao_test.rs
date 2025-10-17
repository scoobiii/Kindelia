// =====================================================
// 🧪 DAO TESTS - Kindelia
// -----------------------------------------------------
// Responsável: @core-dev (Governança / Rust / DAO Layer)
// Objetivo: Testar criação, votação e execução de propostas
// Dependências: src/dao, src/monetization, persistence.rs
// =====================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::{governance::*, treasury::*};

    #[test]
    fn test_create_proposal() {
        let mut dao = DAO::new();
        let prop = dao.create_proposal("Fund HVM Quantum Layer", 1000);
        assert_eq!(prop.status, "Pending");
    }

    #[test]
    fn test_vote_proposal() {
        let mut dao = DAO::new();
        let prop = dao.create_proposal("Add AI Agent", 500);
        dao.vote(prop.id, "yes");
        assert_eq!(dao.get_votes(prop.id).yes, 1);
    }

    #[test]
    fn test_execute_approved() {
        let mut dao = DAO::new();
        let prop = dao.create_proposal("Seed Treasury", 2000);
        dao.vote(prop.id, "yes");
        dao.execute(prop.id);
        assert_eq!(dao.treasury_balance(), 2000);
    }
}
