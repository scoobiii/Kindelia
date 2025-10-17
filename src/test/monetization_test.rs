// Tokenomics & Marketplace Tests
// Responsável: @econ-dev @phd-fintech
// Sprint: 5_Tokenomics

#[test]
fn test_token_minting() {
    let mut token = Token::new("KDL", 1_000_000);
    token.mint("alice", 100);
    assert_eq!(token.balance_of("alice"), 100);
}

#[test]
fn test_transaction_fee_allocation() {
    let mut market = Marketplace::init();
    market.execute_trade("alice", "bob", 50);
    assert!(market.treasury_balance() > 0);
}
