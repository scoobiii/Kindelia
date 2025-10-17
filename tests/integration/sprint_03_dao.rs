// Sprint 03 - DAO Governance Integration Test
// --------------------------------------------
// Responsável: Carla Mendes (QA)
// Objetivo: Validar ciclo completo de governança DAO
// Status: Planejado
// --------------------------------------------
use kindelia::dao::{governance::*, treasury::*};

#[test]
fn test_full_dao_cycle() {
    // 1️⃣ Inicia DAO
    let mut dao = Governance::init();

    // 2️⃣ Submete proposta
    let proposal = dao.submit_proposal("Upgrade HVM 2.0");
    assert!(proposal.id > 0);

    // 3️⃣ Simula votos
    dao.vote(proposal.id, "alice", true);
    dao.vote(proposal.id, "bob", false);

    // 4️⃣ Fecha votação e executa
    let result = dao.execute_proposal(proposal.id);
    assert!(result.success);
}
