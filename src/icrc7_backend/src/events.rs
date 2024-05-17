use crate::{
    common::{
        event_utils::assign_nft_for_event,
        guards::not_anonymous_caller,
        types::{Event, MetadataValue, RequestResult},
        uuid::uuidv4,
    },
    memory::{get_events_collection, insert_event_in_collection, remove_event_from_collection},
};

/// create_event
/// Create an event and insert it in the events collection
///
/// ### arguments
/// * `name` name of the event
/// * `description` description of the event
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
/// * A vector of events with all the events in the collection
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
pub async fn assign_event_to_group(event_id: String, group_id: String) -> RequestResult<Vec<u128>> {
    assign_nft_for_event(event_id, group_id, Some("Basic description".to_string())).await
}
