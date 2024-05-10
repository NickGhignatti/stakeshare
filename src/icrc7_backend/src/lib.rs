use std::collections::HashMap;

use candid::Principal;
use common::{
    event_functions::assign_nft_for_event, group_functions::assign_nft_to_group_member,
    guards::not_anonymous_caller, types::Event, uuid::uuidv4,
};
use ic_cdk::call;
use ic_cdk_macros::export_candid;

pub mod common;
pub mod memory;

use common::types::{Group, Member};
use memory::{
    get_collections, get_events_collection, insert_collection, insert_event_in_collection,
    remove_entry, remove_event_from_collection,
};

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

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn create_event(name: String, description: String) {
    insert_event_in_collection(Event {
        id: uuidv4(),
        title: name,
        description: description,
    });
}

#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn show_events() -> Vec<Event> {
    get_events_collection()
        .iter()
        .map(|(_k, v)| v.clone())
        .collect()
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn remove_event(event_id: String) {
    remove_event_from_collection(event_id);
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn assign_event_to_group(event_id: String, group_id: String) {
    match assign_nft_for_event(event_id, group_id).await {
        Ok(s) => ic_cdk::println!("{}", s),
        Err(e) => ic_cdk::println!("{}", e),
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
