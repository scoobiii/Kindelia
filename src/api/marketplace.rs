use serde::Serialize;

#[derive(Serialize)]
pub struct Listing {
    pub id: String,
    pub energy: u64,
    pub price: u64,
}

pub fn mock_listings() -> Vec<Listing> {
    vec![
        Listing { id: "l1".to_string(), energy: 100, price: 50 },
        Listing { id: "l2".to_string(), energy: 250, price: 120 },
    ]
}
