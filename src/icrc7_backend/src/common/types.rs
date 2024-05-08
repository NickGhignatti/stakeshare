use std::borrow::Cow;

use candid::{CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Clone)]
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

pub type Subaccount = [u8; 32];

#[derive(Serialize, CandidType, Deserialize, Clone, Debug, Copy)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct MintArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    // if None, then the combination of Collection's symbol and token's id will be provided
    // for e.g.: "ICRC7 100"
    pub token_name: Option<String>,
    pub token_description: Option<String>,
    pub token_logo: Option<String>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Arg {
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
}

pub type MintResult = Result<u128, MintError>;

#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum MintError {
    SupplyCapReached,
    Unauthorized,
    TokenIdAlreadyExist,
    TokenIdMinimumLimit,
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}
