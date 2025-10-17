// DAO Governance Unit Tests
// Responsável: @dao-core @security-dev
// Sprint: 4_DAO_Governança

#[test]
fn test_proposal_creation() {
    let dao = Dao::new();
    let prop = dao.create_proposal("Nova feature", "Descrição detalhada", "dev@kindelia.io");
    assert!(prop.is_ok());
}

#[test]
fn test_vote_and_pass_proposal() {
    let mut dao = Dao::mock();
    let prop_id = dao.propose("Atualizar HVM");
    dao.vote(prop_id, "alice", true);
    dao.vote(prop_id, "bob", true);
    assert!(dao.is_approved(prop_id));
}
