use candid::Principal;
use common::{guards::not_anonymous_caller, uuid::uuidv4};
use ic_cdk::call;
use ic_cdk_macros::export_candid;

pub mod common;
pub mod memory;

use common::types::{Group, Member};
use memory::{get_collections, insert_collection};

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn subscribe_group(group: Group) {
    insert_collection(uuidv4(), group);
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
