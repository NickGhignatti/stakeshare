use candid::Principal;
use ic_cdk::call;

use crate::common::guards::not_anonymous_caller;
use crate::common::types::{RequestResult, TransferArg, TransferResult};

/// icrc7_transfer
/// method to transfer some collection token to another identity
///
/// ### arguments
/// * `icrc7_collection_id` principal of the collection containing the token
/// * `args` vector containing all the arguments to permit transfer
/// * `caller` owner of the collection
///
/// ## return
/// Return a custom type containing
/// * `code` numerical code with the result code
/// * `message` a message describing what happened
/// * `body` vector containing the results of the transfers
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
