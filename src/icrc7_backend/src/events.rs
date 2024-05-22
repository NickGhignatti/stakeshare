use crate::{
    common::{
        event_utils::assign_nft_for_event,
        guards::not_anonymous_caller,
        types::{Event, Member, MetadataValue, RequestResult},
        uuid::uuidv4,
    },
    memory::{
        get_event_by_id, get_events_collection, insert_event_in_collection,
        remove_event_from_collection,
    },
};

/// create_event
/// Create an event and insert it in the events collection
///
/// ### arguments
/// * `name` name of the event
/// * `description` description of the event
/// * `metadata` metadata of the event, they will be assigned to the NFTs minted in case the event
///     will be completed by some members
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn create_event(name: String, description: String, metadata: MetadataValue) {
    insert_event_in_collection(Event {
        id: uuidv4(),
        title: name,
        description: description,
        metadata: metadata,
    });
}

/// get_all_events
/// Return all the events collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector containing all the events
#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_all_events() -> RequestResult<Vec<Event>> {
    RequestResult::new(
        200,
        "All events".to_string(),
        get_events_collection()
            .iter()
            .map(|(_k, v)| v.clone())
            .collect(),
    )
}

/// remove_event
/// Remove an event given the id
///
/// ### arguments
/// * `event_id` String representing the event ID
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` generic string describing the result
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub fn remove_event(event_id: String) -> RequestResult<String> {
    remove_event_from_collection(event_id);
    RequestResult::new(200, "Delete of the entry ok".to_string(), String::new())
}

/// assign_event_to_group
/// Assign the event achievement/partecipation ICRC7 NFT to a group
///
/// ### arguments
/// * `event_id` String representing the event ID
/// * `members` Vector of members which should receive
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector containing the token IDs of the NFTs minted
#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn assign_event_to_group(
    event_id: String,
    members: Vec<Member>,
) -> RequestResult<Vec<u128>> {
    assign_nft_for_event(
        event_id.clone(),
        Some(format!(
            "Commemorative NFT for the event {}",
            get_event_by_id(event_id).body.title
        )),
        members,
    )
    .await
}
