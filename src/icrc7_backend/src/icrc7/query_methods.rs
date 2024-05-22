use candid::Principal;
use ic_cdk::call;

use crate::common::types::{Account, Icrc7TokenMetadata, RequestResult};

/// get_icrc7_symbol
/// get the symbol of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` symbol of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_symbol(icrc7_collection_id: Principal) -> RequestResult<String> {
    let (symbol,): (String,) = match call(icrc7_collection_id, "icrc7_symbol", ()).await {
        Ok(value) => value,
        _ => (String::new(),),
    };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        symbol,
    )
}

/// get_icrc7_name
/// get the name of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` name of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_name(icrc7_collection_id: Principal) -> RequestResult<String> {
    let (name,): (String,) = match call(icrc7_collection_id, "icrc7_name", ()).await {
        Ok(value) => value,
        _ => (String::new(),),
    };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        name,
    )
}

/// get_icrc7_description
/// get the description of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` description of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_description(
    icrc7_collection_id: Principal,
) -> RequestResult<Option<String>> {
    let (description,): (Option<String>,) =
        match call(icrc7_collection_id, "icrc7_description", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        description,
    )
}

/// get_icrc7_logo
/// get the logo of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` logo of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_logo(icrc7_collection_id: Principal) -> RequestResult<Option<String>> {
    let (logo,): (Option<String>,) = match call(icrc7_collection_id, "icrc7_logo", ()).await {
        Ok(value) => value,
        _ => (Option::None,),
    };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        logo,
    )
}

/// get_icrc7_total_supply
/// get the total_supply of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` total_supply of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_total_supply(icrc7_collection_id: Principal) -> RequestResult<u128> {
    let (total_supply,): (u128,) = match call(icrc7_collection_id, "icrc7_total_supply", ()).await {
        Ok(value) => value,
        _ => (0,),
    };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        total_supply,
    )
}

/// get_icrc7_supply_cao
/// get the supply_cap of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` supply_cap of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_supply_cap(icrc7_collection_id: Principal) -> RequestResult<Option<u128>> {
    let (supply_cap,): (Option<u128>,) =
        match call(icrc7_collection_id, "crc7_supply_cap", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        supply_cap,
    )
}

/// get_icrc7_max_query_batch_size
/// get the max_query_batch_size of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` max_query_batch_size of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_max_query_batch_size(
    icrc7_collection_id: Principal,
) -> RequestResult<Option<u128>> {
    let (max_query,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_query_batch_size", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        max_query,
    )
}

/// get_icrc7_max_update_batch_size
/// get the max_update_batch_size of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` max_update_batch_size of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_max_update_batch_size(
    icrc7_collection_id: Principal,
) -> RequestResult<Option<u128>> {
    let (max_update,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_update_batch_size", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        max_update,
    )
}

/// get_icrc7_max_take_value
/// get the max_take_value of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` max_take_value of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_max_take_value(
    icrc7_collection_id: Principal,
) -> RequestResult<Option<u128>> {
    let (max_take_value,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_take_value", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        max_take_value,
    )
}

/// get_icrc7_max_memo_size
/// get the max_memo_size of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` max_memo_size of the collection
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_max_memo_size(
    icrc7_collection_id: Principal,
) -> RequestResult<Option<u128>> {
    let (max_memo,): (Option<u128>,) =
        match call(icrc7_collection_id, "icrc7_max_memo_size", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        max_memo,
    )
}

/// get_icrc7_atomic_batch_transfers
/// get the atomic_batch_transfers of an icrc7 collection
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` atomic_batch_transfers of the collection
#[ic_cdk::query(composite = true)]
pub async fn icrc7_atomic_batch_transfers(
    icrc7_collection_id: Principal,
) -> RequestResult<Option<bool>> {
    let (atomic_transfer,): (Option<bool>,) =
        match call(icrc7_collection_id, "icrc7_atomic_batch_transfers", ()).await {
            Ok(value) => value,
            _ => (Option::None,),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        atomic_transfer,
    )
}

/// icrc7_owner_of
/// get the owner of some tokens in a collection
///
/// ### arguments
/// * `ids` token IDs to get the owner
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector of the owners with that token in the collection
#[ic_cdk::query(composite = true)]
pub async fn icrc7_owner_of(
    ids: Vec<u128>,
    icrc7_collection_id: Principal,
) -> RequestResult<Vec<Option<Account>>> {
    let (owners,): (Vec<Option<Account>>,) =
        match call(icrc7_collection_id, "icrc7_atomic_batch_transfers", (ids,)).await {
            Ok(value) => value,
            _ => (vec![],),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        owners,
    )
}

/// icrc7_tokens
/// get the tokens in an icrc7 collection
///
/// ### arguments
/// * `prev` offset of the tokens to take
/// * `take` number of tokens to take
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector with all the token IDs in the collection
#[ic_cdk::query(composite = true)]
pub async fn icrc7_tokens(
    prev: Option<u128>,
    take: Option<u128>,
    icrc7_collection_id: Principal,
) -> RequestResult<Vec<u128>> {
    let (tokens,): (Vec<u128>,) = match call(
        icrc7_collection_id,
        "icrc7_atomic_batch_transfers",
        (prev, take),
    )
    .await
    {
        Ok(value) => value,
        _ => (vec![],),
    };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        tokens,
    )
}

/// get_icrc7_token_metadata
/// get the token_metadata of an icrc7 collection
///
/// ### arguments
/// * `token_ids` vector with the IDs
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector of metadatas (1 for token ID)
#[ic_cdk::query(composite = true)]
pub async fn get_icrc7_token_metadata(
    token_ids: Vec<u128>,
    icrc7_collection_id: Principal,
) -> RequestResult<Vec<Option<Icrc7TokenMetadata>>> {
    let (metadata,): (Vec<Option<Icrc7TokenMetadata>>,) =
        match call(icrc7_collection_id, "icrc7_token_metadata", (token_ids,)).await {
            Ok(value) => value,
            _ => (vec![],),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        metadata,
    )
}

/// get_icrc7_balance_of
/// get the balance of an icrc7 collection
///
/// ### arguments
/// * `accounts` vectors containing the accounts to compute the balance
/// * `icrc7_collection_id` principal of the collection
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector with all the balances (1 for account)
#[ic_cdk::query(composite = true)]
pub async fn icrc7_balance_of(
    accounts: Vec<Account>,
    icrc7_collection_id: Principal,
) -> RequestResult<Vec<u128>> {
    let (balance,): (Vec<u128>,) =
        match call(icrc7_collection_id, "icrc7_balance_of", (accounts,)).await {
            Ok(value) => value,
            _ => (vec![],),
        };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        balance,
    )
}

/// get_icrc7_tokens_of
/// get the tokens of an account in the icrc7 collection
///
/// ### arguments
/// * `account` account owning some tokens in the collection
/// * `prev` offset of the tokens to take
/// * `take` number of tokens to take
/// * `icrc7_collection_id` principal of the collection containing the tokens
///
/// ### return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector containing the IDs of the tokens
#[ic_cdk::query(composite = true)]
pub async fn icrc7_tokens_of(
    account: Account,
    prev: Option<u128>,
    take: Option<u128>,
    icrc7_collection_id: Principal,
) -> RequestResult<Vec<u128>> {
    let (tokens,): (Vec<u128>,) = match call(
        icrc7_collection_id,
        "icrc7_tokens_of",
        (account, prev, take),
    )
    .await
    {
        Ok(value) => value,
        _ => (vec![],),
    };
    RequestResult::new(
        200,
        format!(
            "Correctly retrieved information for the collection with ID = {}",
            icrc7_collection_id
        ),
        tokens,
    )
}
