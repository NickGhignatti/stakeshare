use candid::Principal;

use crate::memory::{get_collections, get_events_collection};

use super::functions::{create_icrc7_collection, mint_icrc7_for_user, update_minting_authority};

use dotenv::dotenv;

pub async fn assign_nft_for_event(event_id: String, group_id: String) -> Result<String, String> {
    let event_collection = get_events_collection();
    let event = match event_collection.get(&event_id) {
        Some(e) => e.clone(),
        _ => {
            return Err(format!(
                "Error while finding the event with ID = {}",
                event_id
            ))
        }
    };
    let group_collection = get_collections();
    let group = match group_collection.get(&group_id) {
        Some(g) => g.clone(),
        _ => {
            return Err(format!(
                "Error while finding the group with ID = {}",
                group_id
            ))
        }
    };
    dotenv().ok();
    let factory_canister_id =
        Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string()).unwrap();
    for member in group.group_members.clone() {
        let icrc7_name = format!(
            "Commemorative NFT for {} to partecipate at the event {}!",
            member.name, event.title
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
