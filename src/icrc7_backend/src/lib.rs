use std::collections::HashMap;

use candid::Principal;
use common::{
    event_functions::assign_nft_for_event,
    functions::group_already_present,
    group_functions::assign_nft_to_group_member,
    guards::not_anonymous_caller,
    types::{Account, Event, OperationCode, TransferArg, TransferResult},
    uuid::uuidv4,
};
use ic_cdk::{call, caller};
use ic_cdk_macros::export_candid;

pub mod common;
pub mod icrc7;
pub mod memory;

use common::types::{Group, Icrc7TokenMetadata, Member};
use memory::{
    get_collections, get_events_collection, get_group_by_id, insert_collection,
    insert_event_in_collection, remove_entry, remove_event_from_collection,
};

/// ## subscribe_group
/// Subrscribe a group into the collection, the subscription include
/// - insertion of the group in the group collection
/// - assign a welcome NFT to each the memebers of the group
///
/// ### arguments
/// * `group` group to add in the collection
///
/// ### return
/// * `400` if there is already a group with the same name
/// * `404` if there is no group with that name
/// * `499` if there has been a minting error for a certain member
/// * `200` if everything is ok
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn subscribe_group(
    mut members: Vec<Member>,
    chef_name: String,
    group_name: String,
) -> OperationCode {
    if group_already_present(group_name.clone()) {
        return OperationCode::DuplicateEntry {
            code: 400,
            message: format!("Duplicate entry for {}", group_name),
        };
    }
    let group_id = uuidv4();
    members.insert(
        members.len(),
        Member {
            name: chef_name,
            internet_identity: caller().to_string(),
        },
    );
    insert_collection(
        group_id.clone(),
        Group {
            group_name: group_name,
            group_members: members,
        },
    );
    assign_nft_to_group_member(group_id).await
}

/// get_user_collections
/// Return all the ICRC7 NFT collection of the user which call the function
///
/// ### return
/// * Hashmap containing Principal of the collection and Principal of the owner
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_user_collections() -> HashMap<Principal, Principal> {
    let caller = ic_cdk::caller();
    ic_cdk::println!("Get user collection caller = {}", caller);
    let factory_canister_id =
        Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string()).unwrap();
    let (all_collections,): (HashMap<Principal, Principal>,) =
        match call(factory_canister_id, "show_collections", ()).await {
            Ok(map) => map,
            _ => (HashMap::new(),),
        };
    for (k, v) in all_collections.clone() {
        ic_cdk::println!("key = {}, value = {}", k, v);
    }
    all_collections
        .iter()
        .filter(|(_k, v)| *v.to_string() == *caller.to_string())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<HashMap<Principal, Principal>>()
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_all_nft_collections() -> HashMap<Principal, Principal> {
    let factory_canister_id =
        Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string()).unwrap();
    let (all_collections,): (HashMap<Principal, Principal>,) =
        match call(factory_canister_id, "show_collections", ()).await {
            Ok(map) => map,
            _ => (HashMap::new(),),
        };
    for (k, v) in all_collections.clone() {
        ic_cdk::println!("key = {}, value = {}", k, v);
    }
    all_collections
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<HashMap<Principal, Principal>>()
}

/// remove_group
/// Remove a group given a specific id
///
/// ### arguments
/// * `group_id` String representing the group ID
///
/// ### return
/// * `404` if the group is not found
/// * `200` if the remotion gone right
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn remove_group(group_id: String) -> OperationCode {
    match get_group_by_id(group_id.clone()) {
        Err(e) => return e,
        _ => {}
    }
    remove_entry(&group_id);
    OperationCode::RemoveOk {
        code: 200,
        message: format!("Remotion of the group {} went ok", group_id),
    }
}

/// remove_all_groups
/// Remove all groups from the collection
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn remove_all_groups() {
    for (k, _) in get_collections() {
        remove_entry(&k);
    }
}

/// get_all_groups
/// Return all the groups present in the collection
///
/// ### return
/// * HashMap containing the tuple ID of the group and the group itself
#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_all_groups() -> HashMap<String, Group> {
    get_collections()
}

/// get_group_members
/// Return all the members of a specific group
///
/// ### arguments
/// * `group_id` String representing the group ID
///
/// ### return
/// * vector containing all the memebers of a group
#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_group_members(group_id: String) -> Vec<Member> {
    match get_collections().get(&group_id) {
        Some(v) => v.group_members.clone(),
        _ => vec![],
    }
}

/// create_event
/// Create an event and insert it in the events collection
///
/// ### arguments
/// * `name` name of the event
/// * `description` description of the event
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn create_event(name: String, description: String) {
    insert_event_in_collection(Event {
        id: uuidv4(),
        title: name,
        description: description,
    });
}

/// get_all_events
/// Return all the events collection
///
/// ### return
/// * A vector of events with all the events in the collection
#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_all_events() -> Vec<Event> {
    get_events_collection()
        .iter()
        .map(|(_k, v)| v.clone())
        .collect()
}

/// remove_event
/// Remove an event given the id
///
/// ### arguments
/// * `event_id` String representing the event ID
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn remove_event(event_id: String) {
    remove_event_from_collection(event_id);
}

/// assign_event_to_group
/// Assign the event achievement/partecipation ICRC7 NFT to a group
///
/// ### arguments
/// * `event_id` String representing the event ID
/// * `group_id` String representing the group ID
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn assign_event_to_group(
    event_id: String,
    group_id: String,
    metadata: Vec<u8>,
) -> OperationCode {
    assign_nft_for_event(
        event_id,
        group_id,
        Some("Basic description".to_string()),
        metadata,
    )
    .await
}

#[ic_cdk::query]
pub fn whoami() -> Principal {
    ic_cdk::caller()
}

export_candid!();
