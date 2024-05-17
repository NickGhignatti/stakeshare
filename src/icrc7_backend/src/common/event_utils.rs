use std::str::FromStr;

use candid::Principal;

use crate::memory::{get_collections, get_events_collection};

use super::{
    types::{OperationCode, RequestResult},
    utils::{create_icrc7_collection, mint_icrc7_for_user, update_minting_authority},
};

use dotenv::dotenv;

pub async fn assign_nft_for_event(
    event_id: String,
    group_id: String,
    icrc7_description: Option<String>,
) -> RequestResult<Vec<u128>> {
    let event_collection = get_events_collection();
    let event = match event_collection.get(&event_id) {
        Some(e) => e.clone(),
        _ => {
            return RequestResult::new(
                404,
                format!("Cannot find event with ID = {}", event_id),
                vec![],
            )
        }
    };

    let group_collection = get_collections();
    let group = match group_collection.get(&group_id) {
        Some(g) => g.clone(),
        _ => {
            return RequestResult::new(
                404,
                format!("Cannot find group with ID = {}", group_id),
                vec![],
            )
        }
    };
    dotenv().ok();
    let factory_canister_id = Principal::from_str(
        option_env!("CANISTER_ID_FACTORY").expect("Env variable CANISTER_ID_FACTORY not found!"),
    )
    .unwrap();
    let mut token_ids = vec![];
    for member in group.group_members.clone() {
        let icrc7_name = format!(
            "Commemorative NFT for {} to partecipate at the event {}!",
            member.name, event.title
        );
        let owner = Principal::from_text(member.internet_identity.clone()).unwrap();
        let icrc7_canister_id = create_icrc7_collection(
            owner.clone(),
            factory_canister_id,
            icrc7_name,
            icrc7_description.clone(),
            Some(event_id.clone()),
        )
        .await;
        // updating minting authority, default is on the factory canister
        update_minting_authority(factory_canister_id, owner.clone(), icrc7_canister_id).await;
        match mint_icrc7_for_user(owner.clone(), icrc7_canister_id).await {
            Ok(v) => token_ids.push(v),
            Err(err) => {
                return RequestResult::new(
                    499,
                    format!(
                        "Error minting NFT for user {} : {:?}",
                        member.name.clone(),
                        err
                    ),
                    vec![],
                )
            }
        };
    }
    RequestResult::new(200, "All NFTs has been minted".to_string(), token_ids)
}
