// ==============================================
// Kindelia Energy - Backend Warp Server
// File: server.rs
// Location: src/server.rs
// Author: Eco Hold
// Description: REST API endpoints for Kindelia Wallet frontend
//   - /login          : User login with private key
//   - /sendTransaction: Send energy / tokens to another address
//   - /marketplace    : List marketplace items
//   - /iot/data       : IoT device telemetry (power, energy today)
//   - /contract/send  : Send smart contract invocation
// Dependencies:
//   - warp
//   - tokio
//   - serde / serde_json
// ==============================================

use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::{NodeRequest, node, hvm, Hash, BlockInfo};
use tokio::sync::oneshot;
use std::sync::Arc;

// ---------- Payloads REST ----------
#[derive(Deserialize)]
pub struct LoginPayload {
    pub privkey: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub address: String,
}

#[derive(Deserialize)]
pub struct SendTxPayload {
    pub to: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct SendTxResponse {
    pub ok: bool,
    pub txid: String,
}

#[derive(Serialize)]
pub struct MarketplaceItem {
    pub id: String,
    pub energy: u64,
    pub price: u64,
}

#[derive(Serialize)]
pub struct IoTData {
    pub power_kw: u64,
    pub energy_today_kwh: u64,
}

#[derive(Deserialize)]
pub struct ContractSendPayload {
    pub contract: String,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize)]
pub struct ContractSendResponse {
    pub ok: bool,
    pub txid: String,
}

// ---------- Server setup ----------
pub async fn start_server(node: Arc<node::NodeHandle>) {
    // Login endpoint
    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |p: LoginPayload| {
            let node = node.clone();
            async move {
                let address = format!("ADDR_{}", &p.privkey[..6]);
                Ok::<_, warp::Rejection>(warp::reply::json(&LoginResponse { address }))
            }
        });

    // Send transaction endpoint
    let send_tx = warp::path("sendTransaction")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |p: SendTxPayload| {
            let node = node.clone();
            async move {
                let txid = format!("tx_{}", rand::random::<u32>());
                Ok::<_, warp::Rejection>(warp::reply::json(&SendTxResponse { ok: true, txid }))
            }
        });

    // Marketplace
    let marketplace = warp::path("marketplace")
        .and(warp::get())
        .and_then(|| async move {
            let items = vec![
                MarketplaceItem { id: "l1".to_string(), energy: 100, price: 50 },
                MarketplaceItem { id: "l2".to_string(), energy: 250, price: 120 },
            ];
            Ok::<_, warp::Rejection>(warp::reply::json(&items))
        });

    // IoT Data
    let iot_data = warp::path!("iot" / "data")
        .and(warp::get())
        .and_then(|| async move {
            let data = IoTData { 
                power_kw: rand::random::<u64>() % 1000, 
                energy_today_kwh: rand::random::<u64>() % 5000 
            };
            Ok::<_, warp::Rejection>(warp::reply::json(&data))
        });

    // Contract send
    let contract_send = warp::path!("contract" / "send")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |p: ContractSendPayload| {
            async move {
                let txid = format!("tx_{}", rand::random::<u32>());
                Ok::<_, warp::Rejection>(warp::reply::json(&ContractSendResponse { ok: true, txid }))
            }
        });

    let routes = login
        .or(send_tx)
        .or(marketplace)
        .or(iot_data)
        .or(contract_send)
        .with(warp::cors().allow_any_origin());

    println!("Server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127,0,0,1], 3030)).await;
}
