use std::str::FromStr;

use candid::Principal;
use ic_cdk::call;

use crate::memory::{get_collections, get_current_token_id, increase_token_id};

use super::types::{Account, Arg, MintArg, MintResult};

pub async fn update_minting_authority(
    factory_id: Principal,
    owner: Principal,
    canister_id: Principal,
) -> bool {
    let (is_up,): (bool,) = call(factory_id, "update_minting_aythority", (canister_id, owner))
        .await
        .unwrap();

    is_up
}

pub async fn create_icrc7_collection(
    owner: Principal,
    factory_id: Principal,
    icrc7_name: String,
    icrc7_description: Option<String>,
    icrc7_logo: Option<String>,
) -> Principal {
    let (result,): (Result<Principal, String>,) = call(
        factory_id,
        "mint_collection_canister",
        (
            Arg {
                icrc7_symbol: format!("ICP"),
                icrc7_name: icrc7_name,
                icrc7_description: icrc7_description,
                icrc7_logo: icrc7_logo,
                icrc7_supply_cap: None,
                icrc7_max_query_batch_size: None,
                icrc7_max_update_batch_size: None,
                icrc7_max_take_value: None,
                icrc7_default_take_value: None,
                icrc7_max_memo_size: None,
                icrc7_atomic_batch_transfers: None,
                tx_window: None,
                permitted_drift: None,
            },
            Account {
                owner: owner,
                subaccount: None,
            },
        ),
    )
    .await
    .unwrap();

    match result {
        Ok(val) => return val,
        _ => return Principal::anonymous(),
    }
}

pub async fn mint_icrc7_for_user(owner: Principal, icrc7_canister_id: Principal) -> MintResult {
    let account = super::types::Account {
        owner: owner,
        subaccount: None,
    };
    let (result,): (MintResult,) = call(
        icrc7_canister_id,
        "icrc7_mint",
        (
            MintArg {
                from_subaccount: None,
                to: account,
                token_id: get_current_token_id(),
                memo: None,
                token_name: None,
                token_description: None,
                token_logo: None,
            },
            owner,
        ),
    )
    .await
    .unwrap();

    increase_token_id();

    result
}

pub fn group_already_present(name: String) -> bool {
    get_collections()
        .iter()
        .filter(|(_k, v)| v.group_name.clone() == name)
        .count()
        > 0
}

pub fn string_to_principal(principal: String) -> Principal {
    match Principal::from_text(principal) {
        Ok(p) => p,
        _ => Principal::anonymous(),
    }
}

pub fn slice_to_principal(principal: &str) -> Principal {
    match Principal::from_str(principal) {
        Ok(p) => p,
        _ => Principal::anonymous(),
    }
}

// pub async fn insert_metadata_ipfs(metadata: Vec<u8>) {
//     let client = IpfsClient::default();
//     let data = Cursor::new(metadata);

//     match client.add(data).await {
//         Ok(res) => ic_cdk::println!("{}", res.hash),
//         Err(e) => ic_cdk::println!("Error reading file: {}", e),
//     }
// }
