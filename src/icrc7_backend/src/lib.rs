use std::collections::HashMap;

use candid::Principal;
use common::{
    group_functions::assign_nft_to_group_member, guards::not_anonymous_caller, uuid::uuidv4,
};
use ic_cdk::call;
use ic_cdk_macros::export_candid;

pub mod common;
pub mod memory;

use common::types::{Group, Member};
use memory::{get_collections, insert_collection, remove_entry};

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn subscribe_group(group: Group) {
    let group_id = uuidv4();
    insert_collection(group_id.clone(), group);
    match assign_nft_to_group_member(group_id).await {
        Ok(v) => ic_cdk::println!("{}", v),
        Err(e) => ic_cdk::println!("{}", e),
    }
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_user_collections() -> HashMap<Principal, Principal> {
    let caller = ic_cdk::caller();
    let factory_canister_id =
        Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string()).unwrap();
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
pub fn remove_group(group_id: String) {
    remove_entry(group_id);
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn remove_all_groups() {
    for (k, _) in get_collections() {
        remove_entry(k);
    }
}

#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn print_groups() -> HashMap<String, Group> {
    get_collections()
}

#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_group_members(group_id: String) -> Vec<Member> {
    match get_collections().get(&group_id) {
        Some(v) => v.group_members.clone(),
        _ => vec![],
    }
}

#[ic_cdk::query]
pub fn whoami() -> Principal {
    ic_cdk::caller()
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn call_canister_whoami(canister_id: Principal) -> String {
    let (result,): (String,) = call(canister_id, "whoami", ()).await.unwrap();
    result.clone()
}

export_candid!();
