use std::borrow::Cow;

use candid::{CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize)]
pub struct Group {
    pub group_name: String,
    pub group_members: Vec<Member>,
}

impl Storable for Group {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(
            serde_json::to_string(self)
                .expect("failed to serialize to bytes")
                .as_bytes()
                .to_vec(),
        )
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let from_str = serde_json::from_str(
            String::from_utf8(bytes.to_vec())
                .expect("failed to serialize from bytes")
                .as_str(),
        );
        from_str.expect("failed to serialize from bytes")
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Clone, Deserialize, Serialize)]
pub struct Member {
    pub name: String,
    pub internet_identity: Principal,
}
