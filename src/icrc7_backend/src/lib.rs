use std::collections::HashMap;

use candid::Principal;
use common::{functions::assign_nft_to_group_member, guards::not_anonymous_caller, uuid::uuidv4};
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

#[ic_cdk::update]
pub fn remove_group(group_id: String) {
    remove_entry(group_id);
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
