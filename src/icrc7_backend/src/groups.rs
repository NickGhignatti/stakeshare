use std::collections::HashMap;

use ic_cdk::caller;

use crate::{
    common::{
        group_utils::assign_nft_to_group_member,
        guards::not_anonymous_caller,
        types::{Account, Group, Member, RequestResult},
        utils::group_already_present,
        uuid::uuidv4,
    },
    memory::{get_collections, get_group_by_id, insert_collection, remove_entry},
};

/// ## subscribe_group
/// Subrscribe a group into the collection, the subscription include
/// - insertion of the group in the group collection
/// - assign a welcome NFT to each the memebers of the group
///
/// ### arguments
/// * `members` vector containing the memebers of the group
/// * `leader_name` name of the group leader
/// * `group_name` name of the group
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector containing the token IDs of the NFTs minted
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn subscribe_group(
    mut members: Vec<Member>,
    leader_name: String,
    group_name: String,
) -> RequestResult<Vec<u128>> {
    if group_already_present(group_name.clone()) {
        return RequestResult::new(400, format!("Duplicate entry for {}", group_name), vec![]);
    }
    let group_id = uuidv4();
    members.insert(
        members.len(),
        Member {
            name: leader_name,
            internet_identity: caller().to_string(),
        },
    );
    insert_collection(
        group_id.clone(),
        Group {
            group_name: group_name,
            group_leader: Account {
                owner: caller(),
                subaccount: None,
            },
            group_members: members,
        },
    );
    assign_nft_to_group_member(group_id).await
}

/// remove_group
/// Remove a group given a specific id
///
/// ### arguments
/// * `group_id` String representing the group ID to remove
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` generic string describing the result
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn remove_group(group_id: String) -> RequestResult<String> {
    let request = get_group_by_id(group_id.clone());
    if request.code != 200 {
        return RequestResult::new(request.code, request.message, String::new());
    }
    remove_entry(&group_id);
    RequestResult::new(200, "Delete of the entry ok".to_string(), String::new())
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
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` hashmap containing the id of the group and the group itself
#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_all_groups() -> RequestResult<HashMap<String, Group>> {
    RequestResult::new(200, "All gorups".to_string(), get_collections())
}

/// get_group_members
/// Return all the members of a specific group
///
/// ### arguments
/// * `group_id` String representing the group ID
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector containing all the members of the group
#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_group_members(group_id: String) -> RequestResult<Vec<Member>> {
    RequestResult::new(
        200,
        format!("Group memebers for {}", group_id),
        match get_collections().get(&group_id) {
            Some(v) => v.group_members.clone(),
            _ => vec![],
        },
    )
}
