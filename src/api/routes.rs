use warp::Filter;
use crate::api::client::{LoginRequest, LoginResponse};
use ed25519_dalek::{Keypair, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use hex;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_login)
}

async fn handle_login(req: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let private_bytes = hex::decode(&req.private_key).map_err(|_| warp::reject())?;
    let secret = SecretKey::from_bytes(&private_bytes).map_err(|_| warp::reject())?;
    let public = PublicKey::from(&secret);
    let mut hasher = Sha256::new();
    hasher.update(public.as_bytes());
    let address = hex::encode(hasher.finalize());
    Ok(warp::reply::json(&LoginResponse { address }))
}
