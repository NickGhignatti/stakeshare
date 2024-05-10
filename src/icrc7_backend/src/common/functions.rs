use candid::Principal;
use ic_cdk::call;

use crate::memory::{get_current_token_id, increase_token_id};

use super::types::{Account, Arg, MintArg, MintResult, TransferArg, TransferResult};

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
) -> Principal {
    let (result,): (Result<Principal, String>,) = call(
        factory_id,
        "mint_collection_canister",
        (
            Arg {
                icrc7_symbol: format!("ICP"),
                icrc7_name: icrc7_name,
                icrc7_description: None,
                icrc7_logo: None,
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

pub async fn transfer(collection_id: Principal, args: Vec<TransferArg>, caller: Principal) {
    let (result,): (Vec<Option<TransferResult>>,) =
        call(collection_id, "icrc7_transfer", (args, caller))
            .await
            .unwrap();
}
