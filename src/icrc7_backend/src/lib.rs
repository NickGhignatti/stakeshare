use ic_cdk_macros::export_candid;

pub mod memory;
pub mod structures;

use memory::{get_collections, insert_collection};
use structures::{Group, Member};

#[ic_cdk::update]
pub fn subscribe_group(group: Group) {
    insert_collection(String::from("Prova"), group);
}

#[ic_cdk::query]
pub fn get_group_members(group_id: String) -> Vec<Member> {
    match get_collections().get(&group_id) {
        Some(v) => v.group_members.clone(),
        _ => vec![],
    }
}

export_candid!();
