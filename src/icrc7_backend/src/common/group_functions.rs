use candid::Principal;

use crate::memory::get_collections;

use super::{
    functions::{create_icrc7_collection, mint_icrc7_for_user, update_minting_authority},
    types::OperationCode,
};

use dotenv::dotenv;

pub async fn assign_nft_to_group_member(uuid: String) -> OperationCode {
    let collection = get_collections();
    let group = match collection.get(&uuid) {
        Some(group) => group,
        // should never trigger this match arm cause the insertion has been done some lines before
        _ => {
            return OperationCode::RetrieveError {
                code: 404,
                message: format!("Not found group with id = {}", uuid),
            }
        }
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
            Err(_) => {
                return OperationCode::MintingError {
                    code: 499,
                    message: format!(
                        "Minting error for the user = {}",
                        member.internet_identity.clone()
                    ),
                }
            }
            _ => {}
        };
    }
    OperationCode::MintOk {
        code: 200,
        message: format!("Minting done for the group {}", group.group_name.clone()),
    }
}
