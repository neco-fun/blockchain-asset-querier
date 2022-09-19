use axum::{response::IntoResponse, routing::get, Json, Router};
use ethers::types::U256;
use reqwest::StatusCode;

use crate::{
    common::{
        self,
        defines::{ContractType, NetworkType},
    },
    contracts::{erc20::ERC20, neco_nft::NecoNFT, neco_stake::NecoStake},
    models::{ERC20Token, NECOStakedInfo},
};

pub fn new_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/neco-staked-info", get(get_neco_staked_info))
        .route("/nft/ownership", get(get_nft_ownership))
        .route("/nft/metadata", get(get_nft_metadata))
        .route("/erc20/balance", get(get_erc20_balance))
}

async fn hello_world() -> &'static str {
    "hello world"
}

async fn get_erc20_balance() -> impl IntoResponse {
    let symbol = ERC20::new(ContractType::NECO, NetworkType::BSCTestNetwork)
        .get_symbol()
        .await
        .unwrap();
    let amount = ERC20::new(ContractType::NECO, NetworkType::BSCTestNetwork)
        .get_balance("0x04a6ae789f1993590268F882F34308E00f9082f9")
        .await
        .unwrap();
    let decimal = ERC20::new(ContractType::NECO, NetworkType::BSCTestNetwork)
        .get_decimal()
        .await
        .unwrap();
    let erc20 = ERC20Token {
        symbol: symbol,
        decimal: decimal,
        amount: amount.to_string(),
    };
    (StatusCode::OK, Json(erc20))
}

async fn get_nft_metadata() -> impl IntoResponse {
    let neco_nft = NecoNFT::new(NetworkType::BSCTestNetwork);
    let ownership = neco_nft
        .get_metadata_by_id(&U256::from_dec_str("10001").unwrap())
        .await
        .unwrap();
    (StatusCode::OK, Json(ownership))
}

async fn get_nft_ownership() -> impl IntoResponse {
    let neco_nft = NecoNFT::new(NetworkType::BSCTestNetwork);
    let ownership = neco_nft
        .get_nft_ownership(
            "0x04a6ae789f1993590268F882F34308E00f9082f9",
            &common::defines::GameClient::NecoFishing,
            &NetworkType::BSCTestNetwork,
        )
        .await
        .unwrap();
    (StatusCode::OK, Json(ownership))
}

async fn get_neco_staked_info() -> impl IntoResponse {
    let staked_amount = NecoStake::new(NetworkType::BSCTestNetwork)
        .get_neco_staked_amount("0x04a6ae789f1993590268F882F34308E00f9082f9")
        .await
        .unwrap();
    let staked_time = NecoStake::new(NetworkType::BSCTestNetwork)
        .get_neco_staked_time("0x04a6ae789f1993590268F882F34308E00f9082f9")
        .await
        .unwrap();

    (
        StatusCode::OK,
        Json(NECOStakedInfo {
            public_address: "0x04a6ae789f1993590268F882F34308E00f9082f9".into(),
            staked_amount: staked_amount.to_string(),
            staked_time: staked_time.to_string(),
        }),
    )
}
