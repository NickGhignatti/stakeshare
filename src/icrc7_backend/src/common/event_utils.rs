use crate::memory::get_events_collection;

use super::{
    types::{Member, RequestResult},
    utils::{
        create_icrc7_collection, mint_icrc7_for_user, slice_to_principal, string_to_principal,
        update_minting_authority,
    },
};

use dotenv::dotenv;

pub async fn assign_nft_for_event(
    event_id: String,
    icrc7_description: Option<String>,
    members: Vec<Member>,
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
    dotenv().ok();
    let factory_canister_id = slice_to_principal(
        option_env!("CANISTER_ID_FACTORY").expect("Env variable CANISTER_ID_FACTORY not found!"),
    );
    let mut token_ids = vec![];
    for member in members {
        let icrc7_name = format!(
            "Commemorative NFT for {} to partecipate at the event {}!",
            member.name, event.title
        );
        let owner = string_to_principal(member.internet_identity.clone());
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
        match mint_icrc7_for_user(
            owner.clone(),
            icrc7_canister_id,
            icrc7_description.clone(),
            Some(event_id.clone()),
        )
        .await
        {
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
