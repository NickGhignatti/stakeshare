use candid::Principal;
use ic_cdk::call;

use crate::common::guards::not_anonymous_caller;
use crate::common::types::{RequestResult, TransferArg, TransferResult};

#[ic_cdk::update(guard = "not_anonymous_caller")]
pub async fn icrc7_transfer(
    icrc7_collection_id: Principal,
    args: Vec<TransferArg>,
    caller: Principal,
) -> RequestResult<Vec<Option<TransferResult>>> {
    let (transfer_results,): (Vec<Option<TransferResult>>,) =
        match call(icrc7_collection_id, "icrc7_transfer", (args, caller)).await {
            Ok(value) => value,
            _ => (vec![],),
        };
    RequestResult::new(
        200,
        format!("Correctly transferred from {}", caller,),
        transfer_results,
    )
}
