use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::BTreeMap as StableBTree;
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static COLLECTIONS: RefCell<StableBTree<String, String, Memory>> = RefCell::new({
        StableBTree::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    });
}

pub fn get_collections() -> HashMap<Principal, Principal> {
    COLLECTIONS.with(|collection| {
        collection
            .borrow()
            .iter()
            .map(|(k, v)| {
                (
                    Principal::from_text(k).unwrap(),
                    Principal::from_text(v).unwrap(),
                )
            })
            .collect()
    })
}

pub fn insert_collection(canister_id: Principal, owner: Principal) {
    COLLECTIONS.with(|collection| {
        collection
            .borrow_mut()
            .insert(owner.to_string(), canister_id.to_string())
    });
}
