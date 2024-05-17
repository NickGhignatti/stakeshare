use std::str::FromStr;

use candid::Principal;

use crate::memory::get_collections;

use super::{
    types::RequestResult,
    utils::{create_icrc7_collection, mint_icrc7_for_user, update_minting_authority},
};

use dotenv::dotenv;

pub async fn assign_nft_to_group_member(uuid: String) -> RequestResult<Vec<u128>> {
    let collection = get_collections();
    let group = match collection.get(&uuid) {
        Some(group) => group,
        // should never trigger this match arm cause the insertion has been done some lines before
        _ => return RequestResult::new(404, format!("Not found group with id = {}", uuid), vec![]),
    };
    dotenv().ok();
    let factory_canister_id = Principal::from_str(
        option_env!("CANISTER_ID_FACTORY").expect("Env variable CANISTER_ID_FACTORY not found!"),
    )
    .unwrap();
    let app_id = ic_cdk::id();
    let mut minted_token: Vec<u128> = vec![];
    for member in group.group_members.clone() {
        let icrc7_name = format!(
            "Commemorative NFT for {} to join {} group!",
            member.name, group.group_name
        );
        let icrc7_canister_id =
            create_icrc7_collection(app_id, factory_canister_id, icrc7_name, None, None).await;
        // updating minting authority, default is on the factory canister
        update_minting_authority(factory_canister_id, app_id, icrc7_canister_id).await;
        match mint_icrc7_for_user(
            Principal::from_text(member.internet_identity.clone()).unwrap(),
            icrc7_canister_id,
        )
        .await
        {
            Err(_) => {
                return RequestResult::new(
                    499,
                    format!("Error minting token for {}", member.name.clone()),
                    vec![],
                )
            }
            Ok(id) => {
                minted_token.push(id);
            }
        };
    }
    RequestResult::new(200, "All NFTs has been minted".to_string(), minted_token)
}
