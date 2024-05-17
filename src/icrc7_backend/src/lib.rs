use std::collections::HashMap;

use candid::Principal;
use common::{
    guards::not_anonymous_caller,
    types::{Account, Event, MetadataValue, RequestResult, TransferArg, TransferResult},
};
use ic_cdk::call;
use ic_cdk_macros::export_candid;

pub mod common;
pub mod events;
pub mod groups;
pub mod icrc7;
pub mod memory;

use common::types::{Group, Icrc7TokenMetadata, Member};

/// get_user_collections
/// Return all the ICRC7 NFT collection of the user which call the function
///
/// ### return
/// * Hashmap containing Principal of the collection and Principal of the owner
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_user_collections() -> HashMap<Principal, Principal> {
    let caller = ic_cdk::caller();

    let factory_canister_id =
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai".to_string()).unwrap();
    let (all_collections,): (HashMap<Principal, Principal>,) =
        match call(factory_canister_id, "show_collections", ()).await {
            Ok(map) => map,
            _ => (HashMap::new(),),
        };
    all_collections
        .iter()
        .filter(|(_k, v)| *v.to_string() == *caller.to_string())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<HashMap<Principal, Principal>>()
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_all_nft_collections() -> HashMap<Principal, Principal> {
    let factory_canister_id =
        Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai".to_string()).unwrap();
    let (all_collections,): (HashMap<Principal, Principal>,) =
        match call(factory_canister_id, "show_collections", ()).await {
            Ok(map) => map,
            _ => (HashMap::new(),),
        };
    all_collections
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<HashMap<Principal, Principal>>()
}

#[ic_cdk::query]
pub fn whoami() -> Principal {
    ic_cdk::caller()
}

export_candid!();
