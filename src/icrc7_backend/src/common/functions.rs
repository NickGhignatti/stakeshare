use candid::Principal;
use ic_cdk::call;
use icrc_ledger_types::icrc1::account::Account;

use crate::memory::get_collections;

use super::types::{Arg, MintArg, MintResult};

use dotenv::dotenv;

pub async fn assign_nft_to_group_member(uuid: String) -> Result<String, String> {
    let collection = get_collections();
    let group = match collection.get(&uuid) {
        Some(group) => group,
        _ => return Err("Cannot find group ID".to_string()),
    };
    dotenv().ok();
    let factory_canister_id =
        Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string()).unwrap();
    for member in group.group_members.clone() {
        let icrc7_name = format!(
            "Commemorative NFT for {} to join {} group!",
            member.name, group.group_name
        );
        let icrc7_canister_id = create_icrc7_collection(
            member.internet_identity.clone(),
            factory_canister_id,
            icrc7_name,
        )
        .await;
        // updating minting authority, default is on the factory canister
        update_minting_authority(
            factory_canister_id,
            member.internet_identity.clone(),
            icrc7_canister_id,
        )
        .await;
        match mint_icrc7_for_user(member.internet_identity.clone(), icrc7_canister_id).await {
            Ok(_value) => {}
            Err(err) => {
                return Err(format!(
                    "Error minting NFT for user {} : {:?}",
                    member.name.clone(),
                    err
                ))
            }
        };
    }
    Ok(String::from("Done"))
}

async fn update_minting_authority(
    factory_id: Principal,
    owner: Principal,
    canister_id: Principal,
) -> bool {
    let (is_up,): (bool,) = call(factory_id, "update_minting_aythority", (canister_id, owner))
        .await
        .unwrap();

    is_up
}

async fn create_icrc7_collection(
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

async fn mint_icrc7_for_user(owner: Principal, icrc7_canister_id: Principal) -> MintResult {
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
                token_id: 10,
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

    result
}
