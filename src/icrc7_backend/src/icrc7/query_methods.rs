use candid::Principal;
use ic_cdk::call;

use crate::common::types::{Account, Icrc7TokenMetadata, RequestResult};

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
