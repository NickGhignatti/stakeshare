use std::collections::HashMap;

use candid::{Encode, Principal};
use common::guards::not_anonymous_caller;
use common::types::{Arg, InitArg};
use ic_cdk::api::management_canister::{
    main::{create_canister, install_code, CreateCanisterArgument, InstallCodeArgument},
    provisional::CanisterSettings,
};
use ic_cdk::call;
use ic_cdk_macros::export_candid;
use icrc_ledger_types::icrc1::account::Account;
use memory::{get_collections, insert_collection};

pub const ICRC7_WASM: &[u8] = std::include_bytes!("../../../wasm_files/icrc7.wasm.gz");
pub mod common;
pub mod memory;

#[ic_cdk::update(guard = "not_anonymous_caller")]
async fn mint_collection_canister(arg: Arg, minting_account: Account) -> Result<Principal, String> {
    // let caller = ic_cdk::caller();
    let account = minting_account;
    ic_cdk::println!("Account = {}", account.owner.clone());
    let principal = match create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::id(), account.clone().owner]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
                reserved_cycles_limit: None,
            }),
        },
        7_692_307_692 + 6_153_895_378, // Basic cycles amount to create a canister + Basic cycles amount to tip the craeted canister
    )
    .await
    {
        Err((code, msg)) => return Err(format!("Rejection Code: {:?}, Message: {:?}", code, msg)),
        Ok((principal,)) => principal.canister_id,
    };
    let init_arg = InitArg::from((account, arg));
    let init_arg = Encode!(&init_arg).unwrap();
    match install_code(InstallCodeArgument {
        mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Install,
        canister_id: principal,
        wasm_module: ICRC7_WASM.to_vec(),
        arg: init_arg,
    })
    .await
    {
        Ok(()) => {
            insert_collection(account.owner.clone(), principal);
            Ok(principal)
        }
        Err((code, msg)) => Err(format!("Code: {:?}, Message: {:?}", code, msg)),
    }
}

#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn show_collections() -> HashMap<Principal, Principal> {
    get_collections()
}

#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn get_user_collections(caller: Principal) -> Vec<Principal> {
    get_collections()
        .iter()
        .filter(|(_k, v)| *v.to_string() == *caller.to_string())
        .map(|(k, _v)| (k.clone()))
        .collect::<Vec<Principal>>()
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn update_minting_aythority(canister_id: Principal, owner: Principal) -> bool {
    let (is_updated,): (bool,) = call(
        canister_id,
        "icrc7_set_minting_authority",
        (Account {
            owner: owner,
            subaccount: None,
        },),
    )
    .await
    .unwrap();

    is_updated
}

#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn whoami(caller: Principal) -> String {
    caller.to_string()
}

#[ic_cdk::query(guard = "not_anonymous_caller")]
pub fn check_collection_ownership(collection_id: Principal, owner: Principal) -> bool {
    match get_collections().get(&collection_id) {
        Some(value) => {
            if value.to_string() == owner.to_string() {
                return true;
            }
            return false;
        }
        _ => return false,
    }
}

export_candid!();
