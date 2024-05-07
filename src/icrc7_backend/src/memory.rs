use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::BTreeMap as StableBTree;
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::structures::Group;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static COLLECTIONS: RefCell<StableBTree<String, Group, Memory>> = RefCell::new({
        StableBTree::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    });
}

pub fn get_collections() -> HashMap<String, Group> {
    COLLECTIONS.with(|collection| collection.borrow().iter().collect())
}

pub fn insert_collection(group_id: String, group: Group) {
    COLLECTIONS.with(|collection| collection.borrow_mut().insert(group_id, group));
}
