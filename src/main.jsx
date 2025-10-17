// ==============================================
// Kindelia Energy - Backend Warp API
// Author: Eco Hold
// Description: Endpoints para login, transações,
// marketplace, IoT e smart contracts HVM.
// ==============================================

use warp::Filter;
use crate::api::client::{LoginRequest, LoginResponse, TransactionRequest, TransactionResponse};
use crate::api::marketplace::{mock_listings, Listing};
use crate::api::iot::{mock_iot_data, IoTData};
use crate::api::contract::{ContractRequest, ContractResponse, mock_contract_send};
use sha2::{Sha256, Digest};
use ed25519_dalek::{SecretKey, PublicKey};
use hex;

mod api;

#[tokio::main]
async fn main() {
    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_login);

    let send_tx = warp::path("sendTransaction")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_send_transaction);

    let marketplace = warp::path("marketplace")
        .and(warp::get())
        .and_then(handle_marketplace);

    let iot_data = warp::path!("iot" / "data")
        .and(warp::get())
        .and_then(handle_iot_data);

    let contract_send = warp::path!("contract" / "send")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_contract_send);

    let routes = login
        .or(send_tx)
        .or(marketplace)
        .or(iot_data)
        .or(contract_send)
        .with(warp::cors().allow_any_origin());

    println!("Kindelia backend running on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// ---------- Handlers ----------

async fn handle_login(req: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let private_bytes = hex::decode(&req.private_key).map_err(|_| warp::reject())?;
    let secret = SecretKey::from_bytes(&private_bytes).map_err(|_| warp::reject())?;
    let public = PublicKey::from(&secret);
    let mut hasher = Sha256::new();
    hasher.update(public.as_bytes());
    let address = hex::encode(hasher.finalize());
    Ok(warp::reply::json(&LoginResponse { address }))
}

async fn handle_send_transaction(req: TransactionRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let txid = format!("tx_{}", rand::random::<u32>());
    Ok(warp::reply::json(&TransactionResponse { txid, ok: true }))
}

async fn handle_marketplace() -> Result<impl warp::Reply, warp::Rejection> {
    let listings: Vec<Listing> = mock_listings();
    Ok(warp::reply::json(&listings))
}

async fn handle_iot_data() -> Result<impl warp::Reply, warp::Rejection> {
    let data: IoTData = mock_iot_data();
    Ok(warp::reply::json(&data))
}

async fn handle_contract_send(req: ContractRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let res: ContractResponse = mock_contract_send(&req);
    Ok(warp::reply::json(&res))
}
