use std::collections::HashMap;

use candid::Principal;
use common::types::{Account, Event, MetadataValue, RequestResult, TransferArg, TransferResult};
use ic_cdk_macros::export_candid;

pub mod common;
pub mod events;
pub mod groups;
pub mod icrc7;
pub mod memory;

use common::types::{Group, Icrc7TokenMetadata, Member};

#[ic_cdk::query]
pub fn whoami() -> Principal {
    ic_cdk::caller()
}

export_candid!();
