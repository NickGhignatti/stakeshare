use std::str::FromStr;

use crate::common::guards::not_anonymous_caller;
use candid::Principal;
use dotenv::dotenv;
use ic_cdk::{call, caller};
use icrc_ledger_types::icrc1::account::Account;

pub mod query_methods;
pub mod update_methods;

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_user_collection() -> Vec<u128> {
    let caller = caller();
    dotenv().ok();
    let factory_canister_id = Principal::from_str(
        option_env!("CANISTER_ID_FACTORY").expect("Env variable CANISTER_ID_FACTORY not found!"),
    )
    .unwrap_or(Principal::anonymous());
    let (icrc7_collections,): (Vec<Principal>,) =
        match call(factory_canister_id, "get_user_collections", (caller,)).await {
            Ok(r) => r,
            _ => (vec![],),
        };
    let mut tokens: Vec<u128> = vec![];
    for collection in icrc7_collections.clone() {
        let (mut partial_tokens,): (Vec<u128>,) = match call(
            collection,
            "icrc7_tokens_of",
            (
                Account {
                    owner: caller,
                    subaccount: None,
                },
                None::<u128>,
                None::<u128>,
            ),
        )
        .await
        {
            Ok(r) => r,
            _ => (vec![],),
        };
        tokens.append(&mut partial_tokens);
    }
    tokens
}
