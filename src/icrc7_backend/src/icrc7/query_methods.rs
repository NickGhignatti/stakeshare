use candid::Principal;
use ic_cdk::call;

use crate::common::types::{Account, Icrc7TokenMetadata};

#[ic_cdk::update]
pub async fn get_icrc7_symbol(icrc7_collection_id: Principal) -> String {
    let (symbol,): (String,) = match call(icrc7_collection_id, "icrc7_symbol", ()).await {
        Ok(value) => value,
        _ => (String::new(),),
    };
    symbol
}

#[ic_cdk::update]
pub async fn get_icrc7_name(icrc7_collection_id: Principal) -> String {
    let (name,): (String,) = match call(icrc7_collection_id, "icrc7_name", ()).await {
        Ok(value) => value,
        _ => (String::new(),),
    };
    name
}

#[ic_cdk::update]
pub async fn get_icrc7_description(icrc7_collection_id: Principal) -> Option<String> {
    let (description,): (Option<String>,) =
        match call(icrc7_collection_id, "icrc7_description", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    description
}

#[ic_cdk::update]
pub async fn get_icrc7_logo(icrc7_collection_id: Principal) -> Option<String> {
    let (logo,): (Option<String>,) = match call(icrc7_collection_id, "icrc7_logo", ()).await {
        Ok(value) => value,
        _ => (Option::None,),
    };
    logo
}

#[ic_cdk::update]
pub async fn get_icrc7_total_supply(icrc7_collection_id: Principal) -> u128 {
    let (total_supply,): (u128,) = match call(icrc7_collection_id, "icrc7_total_supply", ()).await {
        Ok(value) => value,
        _ => (0,),
    };
    total_supply
}

#[ic_cdk::update]
pub async fn get_icrc7_supply_cap(icrc7_collection_id: Principal) -> Option<u128> {
    let (supply_cap,): (Option<u128>,) =
        match call(icrc7_collection_id, "crc7_supply_cap", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    supply_cap
}

#[ic_cdk::update]
pub async fn get_icrc7_max_query_batch_size(icrc7_collection_id: Principal) -> Option<u128> {
    let (max_query,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_query_batch_size", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    max_query
}

#[ic_cdk::update]
pub async fn get_icrc7_max_update_batch_size(icrc7_collection_id: Principal) -> Option<u128> {
    let (max_update,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_update_batch_size", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    max_update
}

#[ic_cdk::update]
pub async fn get_icrc7_max_take_value(icrc7_collection_id: Principal) -> Option<u128> {
    let (mac_take_value,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_take_value", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    mac_take_value
}

#[ic_cdk::update]
pub async fn get_icrc7_max_memo_size(icrc7_collection_id: Principal) -> Option<u128> {
    let (max_memo,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_memo_size", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    max_memo
}

#[ic_cdk::update]
pub async fn icrc7_atomic_batch_transfers(icrc7_collection_id: Principal) -> Option<bool> {
    let (atomic_transfer,): (Option<bool>,) =
        match call(icrc7_collection_id, "icrc7_atomic_batch_transfers", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    atomic_transfer
}

#[ic_cdk::update]
pub async fn icrc7_owner_of(
    ids: Vec<u128>,
    icrc7_collection_id: Principal,
) -> Vec<Option<Account>> {
    let (owners,): (Vec<Option<Account>>,) =
        match call(icrc7_collection_id, "icrc7_atomic_batch_transfers", (ids,)).await {
            Ok(value) => value,
            _ => (vec![],),
        };
    owners
}

#[ic_cdk::update]
pub async fn icrc7_tokens(
    prev: Option<u128>,
    take: Option<u128>,
    icrc7_collection_id: Principal,
) -> Vec<u128> {
    let (tokens,): (Vec<u128>,) = match call(
        icrc7_collection_id,
        "icrc7_atomic_batch_transfers",
        (prev, take),
    )
    .await
    {
        Ok(value) => value,
        _ => (vec![],),
    };
    tokens
}

#[ic_cdk::update]
pub async fn get_icrc7_token_metadata(
    token_ids: Vec<u128>,
    icrc7_collection_id: Principal,
) -> Vec<Option<Icrc7TokenMetadata>> {
    let (metadata,): (Vec<Option<Icrc7TokenMetadata>>,) =
        match call(icrc7_collection_id, "icrc7_token_metadata", (token_ids,)).await {
            Ok(value) => value,
            _ => (vec![],),
        };
    metadata
}

#[ic_cdk::update]
pub async fn icrc7_balance_of(accounts: Vec<Account>, icrc7_collection_id: Principal) -> Vec<u128> {
    let (balance,): (Vec<u128>,) =
        match call(icrc7_collection_id, "icrc7_balance_of", (accounts,)).await {
            Ok(value) => value,
            _ => (vec![],),
        };
    balance
}

#[ic_cdk::update]
pub async fn icrc7_tokens_of(
    account: Account,
    prev: Option<u128>,
    take: Option<u128>,
    icrc7_collection_id: Principal,
) -> Vec<u128> {
    let (tokens,): (Vec<u128>,) = match call(
        icrc7_collection_id,
        "icrc7_tokens_of",
        (account, prev, take),
    )
    .await
    {
        Ok(value) => value,
        _ => (vec![],),
    };
    tokens
}
