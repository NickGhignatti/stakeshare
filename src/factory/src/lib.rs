use std::collections::HashMap;

use candid::{Encode, Principal};
use ic_cdk::{
    api::{
        call::{self, result},
        management_canister::{
            main::{create_canister, install_code, CreateCanisterArgument, InstallCodeArgument},
            provisional::CanisterSettings,
        },
    },
    call,
};
use ic_cdk_macros::export_candid;
use icrc_ledger_types::icrc1::account::Account;
use memory::{get_collections, insert_collection};
use types::{Arg, InitArg};

pub const ICRC7_WASM: &[u8] = std::include_bytes!("../../../wasm_files/icrc7.wasm.gz");
pub mod memory;
pub mod types;

#[ic_cdk::update]
async fn mint_collection_canister(arg: Arg) -> Result<Principal, String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Anonymous Caller".into());
    }
    let account = Account {
        owner: caller.clone(),
        subaccount: None,
    };
    ic_cdk::println!("{:?}", arg);
    let principal = match create_canister(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::id(), caller.clone()]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
                reserved_cycles_limit: None,
            }),
        },
        7_692_307_692 + 6_153_895_378,
    )
    .await
    {
        Err((code, msg)) => {
            ic_cdk::println!("Error after await!");
            return Err(format!("Rejection Code: {:?}, Message: {:?}", code, msg));
        }
        Ok((principal,)) => principal.canister_id,
    };
    ic_cdk::println!("Principal = {}", principal);
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
            insert_collection(caller.clone(), principal);
            Ok(principal)
        }
        Err((code, msg)) => {
            ic_cdk::println!("Error after second await");
            Err(format!("Code: {:?}, Message: {:?}", code, msg))
        }
    }
}

#[ic_cdk::query]
pub fn show_collections() -> HashMap<String, String> {
    get_collections()
}

#[ic_cdk::update]
pub async fn call_whoami() -> Vec<String> {
    let mut whoami_vector: Vec<String> = vec![];
    for (k, _v) in get_collections() {
        let (result,): (String,) = call(Principal::from_text(k).unwrap(), "whoami", ())
            .await
            .unwrap();
        ic_cdk::println!("Result of the Factory canister = {}", result);
        whoami_vector.push(result.clone());
    }
    whoami_vector
}

#[ic_cdk::query]
pub fn whoami(caller: Principal) -> String {
    caller.to_string()
}

export_candid!();
