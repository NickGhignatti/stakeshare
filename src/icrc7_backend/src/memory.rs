use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::BTreeMap as StableBTree;
use ic_stable_structures::Cell as StableCell;
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::common::types::{Event, Group, RequestResult};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static COLLECTIONS: RefCell<StableBTree<String, Group, Memory>> = RefCell::new({
        StableBTree::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    });

    static EVENT_COLLECTIONS: RefCell<StableBTree<String, Event, Memory>> = RefCell::new({
        StableBTree::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    });

    static TOKEN_COUNTER: RefCell<StableCell<u128, Memory>> = RefCell::new({
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0).unwrap()
    })
}

pub fn get_collections() -> HashMap<String, Group> {
    COLLECTIONS.with(|collection| collection.borrow().iter().collect())
}

pub fn insert_collection(group_id: String, group: Group) {
    COLLECTIONS.with(|collection| collection.borrow_mut().insert(group_id, group));
}

pub fn remove_entry(group_id: &String) {
    COLLECTIONS.with(|collection| collection.borrow_mut().remove(group_id));
}

pub fn get_group_by_id(group_id: String) -> RequestResult<Group> {
    match COLLECTIONS.with(|collection| collection.borrow().get(&group_id)) {
        Some(g) => RequestResult::new(200, format!("Correctly retrieved group {}", group_id), g),
        _ => RequestResult::new(
            404,
            format!("Not found group {}", group_id),
            Group::default(),
        ),
    }
}

pub fn get_events_collection() -> HashMap<String, Event> {
    EVENT_COLLECTIONS.with(|collection| collection.borrow().iter().collect())
}

pub fn insert_event_in_collection(event: Event) {
    EVENT_COLLECTIONS.with(|collection| collection.borrow_mut().insert(event.id.clone(), event));
}

pub fn remove_event_from_collection(event_id: String) {
    EVENT_COLLECTIONS.with(|collection| collection.borrow_mut().remove(&event_id));
}

pub fn get_event_by_id(event_id: String) -> RequestResult<Event> {
    match EVENT_COLLECTIONS.with(|collection| collection.borrow_mut().get(&event_id)) {
        Some(e) => RequestResult::new(200, format!("Correctly retrieved event {}", event_id), e),
        _ => RequestResult::new(
            404,
            format!("Not found event {}", event_id),
            Event::default(),
        ),
    }
}

pub fn get_current_token_id() -> u128 {
    TOKEN_COUNTER.with(|token| token.borrow().get().clone())
}

pub fn increase_token_id() {
    let old_token_id = get_current_token_id();
    let _ = TOKEN_COUNTER.with(|token| token.borrow_mut().set(old_token_id + 1));
}
