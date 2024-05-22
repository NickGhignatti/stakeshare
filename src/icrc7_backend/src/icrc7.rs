use self::query_methods::get_icrc7_logo;
use crate::{
    common::{
        guards::not_anonymous_caller,
        types::{Icrc7TokenMetadata, MetadataValue, RequestResult},
        utils::{slice_to_principal, string_to_principal},
    },
    memory::get_event_by_id,
};
use candid::Principal;
use dotenv::dotenv;
use ic_cdk::{call, caller};
use icrc_ledger_types::icrc1::account::Account;
use std::collections::HashMap;

pub mod query_methods;
pub mod update_methods;

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_user_tokens_collection() -> RequestResult<HashMap<u128, String>> {
    let caller = caller();
    dotenv().ok();
    let factory_canister_id = slice_to_principal(
        option_env!("CANISTER_ID_FACTORY").expect("Env variable CANISTER_ID_FACTORY not found!"),
    );
    let (icrc7_collections,): (Vec<Principal>,) =
        match call(factory_canister_id, "get_user_collections", (caller,)).await {
            Ok(r) => r,
            _ => (vec![],),
        };
    let mut tokens: HashMap<u128, String> = HashMap::new();
    for collection in icrc7_collections.clone() {
        let (partial_tokens,): (Vec<u128>,) = match call(
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
        for t in partial_tokens {
            tokens.insert(t, collection.to_string());
        }
    }
    RequestResult::new(
        200,
        format!("Collection found for the user {}", caller),
        tokens,
    )
}

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn get_token_metadata(
    token_id: u128,
    collection_id: String,
) -> RequestResult<Vec<MetadataValue>> {
    let collection_id = string_to_principal(collection_id);
    let (token_metadatas,): (Vec<Option<Icrc7TokenMetadata>>,) =
        match call(collection_id, "icrc7_token_metadata", (&[token_id],)).await {
            Ok(meta) => meta,
            _ => (vec![],),
        };
    let mut resulting_metadata: Vec<MetadataValue> = vec![];
    for metadata in token_metadatas {
        match metadata {
            Some(hash_map) => {
                for (k, v) in hash_map {
                    if k == "logo".to_string() {
                        let logo_id = match get_icrc7_logo(collection_id).await.body {
                            Some(path) => path,
                            _ => String::new(),
                        };
                        let event = get_event_by_id(logo_id);
                        if event.code == 200 {
                            resulting_metadata.push(event.body.metadata)
                        }
                    } else {
                        resulting_metadata.push(v)
                    }
                }
            }
            _ => {}
        }
    }
    RequestResult::new(
        200,
        format!("Metadata for the token {} found!", token_id),
        resulting_metadata,
    )
}
