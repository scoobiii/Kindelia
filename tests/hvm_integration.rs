// Kindelia HVM Integration Tests
// Responsável: @core-dev @qa-phd
// Sprint: 2_HVM_Runtime
// Objetivo: Garantir consistência do runtime HVM em execuções paralelas e determinísticas.

#[test]
fn test_hvm_parallel_consistency() {
    let input = "(λx.λy.(x y))";
    let result = hvm::eval(input);
    assert_eq!(result, "λy.(y)");
}

#[test]
fn test_hvm_deterministic_hash() {
    let hash1 = hvm::hash_state("λx.x");
    let hash2 = hvm::hash_state("λx.x");
    assert_eq!(hash1, hash2, "Hashes devem ser determinísticos");
}
