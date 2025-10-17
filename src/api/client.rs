// ==============================================
// Kindelia Energy - API Client
// File: client.rs
// Location: src/client.rs
// Author: Eco Hold
// Description: API client for interacting with Kindelia backend Warp server
// Endpoints supported:
//   - /login
//   - /sendTransaction
//   - /marketplace
//   - /iot/data
//   - /contract/send
// ==============================================

#![allow(dead_code)]

use std::fmt::Debug;
use std::ops::Deref;

use reqwest::{Client, IntoUrl, Method, RequestBuilder, Url};
use serde::{Serialize, Deserialize};

use crate::hvm::{self, Term};
use crate::net::ProtoComm;
use crate::node;

use super::{BlockInfo, CtrInfo, FuncInfo, Hash, HexStatement, Name, RegInfo, Stats};

// ---------- Payloads ----------
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub private_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequest {
    pub to: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub txid: String,
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketplaceItem {
    pub id: String,
    pub energy: u64,
    pub price: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IoTData {
    pub power_kw: u64,
    pub energy_today_kwh: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractSendPayload {
    pub contract: String,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractSendResponse {
    pub txid: String,
    pub ok: bool,
}

// ---------- Client ----------
pub struct ApiClient {
    client: reqwest::Client,
    base_url: Url,
}

impl Deref for ApiClient {
    type Target = Url;
    fn deref(&self) -> &Url {
        &self.base_url
    }
}

type ApiResult<T> = Result<T, String>;

impl ApiClient {
    pub fn new<U>(base_url: U, client: Option<Client>) -> Result<Self, reqwest::Error>
    where
        U: IntoUrl + Debug,
    {
        let url_txt = format!("{:?}", base_url);
        let base_url = base_url
            .into_url()
            .unwrap_or_else(|_| panic!("Invalid base URL: '{}'.", url_txt));
        let client = client.unwrap_or_else(Client::new);
        Ok(ApiClient { client, base_url })
    }

    fn base_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = self
            .base_url
            .join(path)
            .unwrap_or_else(|err| panic!("Invalid URL sub-path '{}'; {}", path, err));
        self.client.request(method, url)
    }

    async fn req<T, B>(&self, method: Method, path: &str, body: Option<B>) -> ApiResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let req = self.base_request(method, path);
        let req = if let Some(body) = body { req.json(&body) } else { req };
        let res = req.send().await.map_err(|e| e.to_string())?;
        if res.status().is_success() {
            res.json::<T>().await.map_err(|e| e.to_string())
        } else {
            let status = res.status();
            let text = res.text().await.map_err(|e| e.to_string())?;
            Err(format!("Error {}: {}", status, text))
        }
    }

    pub async fn get<T>(&self, path: &str) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        self.req::<T, String>(Method::GET, path, None).await
    }

    // ---------- Existing HVM / Node endpoints ----------
    pub async fn get_stats(&self) -> ApiResult<Stats> {
        self.get::<Stats>("/stats").await
    }

    pub async fn get_blocks(&self) -> ApiResult<Vec<BlockInfo>> {
        self.get::<Vec<BlockInfo>>("/blocks").await
    }

    pub async fn get_block_hash(&self, index: u64) -> ApiResult<String> {
        self.get::<String>(&format!("/block-hash/{}", index)).await
    }

    pub async fn get_block(&self, id: Hash) -> ApiResult<Option<BlockInfo>> {
        self.get::<Option<BlockInfo>>(&format!("/blocks/{}", id)).await
    }

    pub async fn get_functions(&self) -> ApiResult<Vec<Name>> {
        self.get::<Vec<Name>>("/functions").await
    }

    pub async fn get_function(&self, name: Name) -> ApiResult<FuncInfo> {
        self.get::<FuncInfo>(&format!("/functions/{}", name)).await
    }

    pub async fn get_function_state(&self, name: Name) -> ApiResult<Term> {
        self.get::<Term>(&format!("/functions/{}/state", name)).await
    }

    pub async fn get_constructor(&self, name: Name) -> ApiResult<CtrInfo> {
        self.get::<CtrInfo>(&format!("/constructor/{}", name)).await
    }

    pub async fn run_code(&self, code: Vec<HexStatement>) -> ApiResult<Vec<hvm::StatementInfo>> {
        self.req(Method::POST, "/run", Some(code)).await
    }

    pub async fn publish_code(&self, code: Vec<HexStatement>) -> ApiResult<Vec<Result<(), ()>>> {
        self.req(Method::POST, "/publish", Some(code)).await
    }

    pub async fn get_peers<C: ProtoComm>(&self, all: bool) -> ApiResult<Vec<node::Peer<C::Address>>>
    where
        C::Address: serde::de::DeserializeOwned,
    {
        if all {
            self.get::<Vec<node::Peer<C::Address>>>("/peers/all").await
        } else {
            self.get::<Vec<node::Peer<C::Address>>>("/peers").await
        }
    }

    pub async fn get_reg_info(&self, name: &str) -> ApiResult<RegInfo> {
        self.get::<RegInfo>(&format!("/reg/{}", name)).await
    }

    // ---------- New endpoints for frontend ----------
    pub async fn login(&self, private_key: &str) -> ApiResult<LoginResponse> {
        let body = LoginRequest { private_key: private_key.to_string() };
        self.req::<LoginResponse, _>(Method::POST, "/login", Some(body)).await
    }

    pub async fn send_transaction(&self, to: &str, amount: u64) -> ApiResult<TransactionResponse> {
        let body = TransactionRequest { to: to.to_string(), amount };
        self.req::<TransactionResponse, _>(Method::POST, "/sendTransaction", Some(body)).await
    }

    pub async fn marketplace(&self) -> ApiResult<Vec<MarketplaceItem>> {
        self.get::<Vec<MarketplaceItem>>("/marketplace").await
    }

    pub async fn iot_data(&self) -> ApiResult<IoTData> {
        self.get::<IoTData>("/iot/data").await
    }

    pub async fn contract_send(
        &self,
        contract: &str,
        method: &str,
        params: serde_json::Value,
    ) -> ApiResult<ContractSendResponse> {
        let body = ContractSendPayload {
            contract: contract.to_string(),
            method: method.to_string(),
            params,
        };
        self.req::<ContractSendResponse, _>(Method::POST, "/contract/send", Some(body)).await
    }
}
