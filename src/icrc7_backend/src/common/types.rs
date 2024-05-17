use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Int, Nat, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Group {
    pub group_name: String,
    pub group_leader: Account,
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

#[derive(CandidType, Clone, Deserialize, Serialize, Debug)]
pub struct Member {
    pub name: String,
    pub internet_identity: String,
}

pub type Subaccount = [u8; 32];

#[derive(Serialize, CandidType, Deserialize, Clone, Debug, Copy)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(Serialize, CandidType, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub metadata: MetadataValue,
}

impl Storable for Event {
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

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

pub type TransferResult = Result<u128, TransferError>;
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

#[derive(CandidType, Clone, Debug)]
pub enum OperationCode {
    MintOk { code: u16, message: String },
    RemoveOk { code: u16, message: String },
    RetrieveError { code: u16, message: String },
    InsertError { code: u16, message: String },
    DuplicateEntry { code: u16, message: String },
    MintingError { code: u16, message: String },
}

/// Variant type for the `metadata` endpoint values.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MetadataValue {
    Nat(Nat),
    Int(Int),
    Text(String),
    Blob(ByteBuf),
}

#[derive(CandidType, Clone, Debug)]
pub struct RequestResult<T> {
    pub code: u16,
    pub message: String,
    pub body: T,
}

impl<T> RequestResult<T> {
    pub fn new(code: u16, message: String, body: T) -> RequestResult<T> {
        RequestResult {
            code: code,
            message: message,
            body: body,
        }
    }
}

pub type Icrc7TokenMetadata = HashMap<String, MetadataValue>;
