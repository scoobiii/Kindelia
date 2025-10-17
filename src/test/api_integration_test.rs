// API Integration Tests
// Responsável: @net-dev @qa-devops
// Sprint: 3_API_Persistencia

#[test]
fn test_rpc_health_check() {
    let status = ApiClient::get("/health");
    assert_eq!(status, 200);
}

#[test]
fn test_post_transaction() {
    let payload = r#"{ "from": "alice", "to": "bob", "amount": 10 }"#;
    let res = ApiClient::post("/tx", payload);
    assert_eq!(res.status(), 201);
}
