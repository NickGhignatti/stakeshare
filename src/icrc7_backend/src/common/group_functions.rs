use candid::Principal;

use crate::memory::get_collections;

use super::functions::{create_icrc7_collection, mint_icrc7_for_user, update_minting_authority};

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
