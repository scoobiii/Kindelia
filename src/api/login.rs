// ==============================================
// Kindelia Energy - API de Autenticação
// Author: Eco Hold
// Description: Endpoints Warp para login de usuários
//   residencial, comercial, industrial e usinas.
//   Pré-login valida chave privada; pós-login gera
//   endereço e token JWT.
// Dependencies:
//   - warp
//   - ed25519-dalek
//   - sha2
//   - hex
// ==============================================

use warp::Filter;
use crate::api::client::{LoginRequest, LoginResponse};
use ed25519_dalek::{Keypair, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use hex;

// Definição das rotas
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(pre_login)
        .and_then(handle_login)
}

// Pré-login: valida chave, prepara nonce
async fn pre_login(req: LoginRequest) -> Result<LoginRequest, warp::Rejection> {
    if req.private_key.len() != 64 {
        return Err(warp::reject());
    }
    // Opcional: gerar nonce ou verificar usuário
    Ok(req)
}

// Pós-login: gera endereço, retorna resposta
async fn handle_login(req: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let private_bytes = hex::decode(&req.private_key).map_err(|_| warp::reject())?;
    let secret = SecretKey::from_bytes(&private_bytes).map_err(|_| warp::reject())?;
    let public = PublicKey::from(&secret);
    let mut hasher = Sha256::new();
    hasher.update(public.as_bytes());
    let address = hex::encode(hasher.finalize());

    // Aqui você pode criar sessão ou token JWT
    Ok(warp::reply::json(&LoginResponse { address }))
}
