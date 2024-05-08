use candid::Principal;

use crate::{
    errors::InsertTransactionError,
    guards::owner_guard,
    state::{call_sync_logs, STATE},
    ApprovalArg, ApproveResult, BurnArg, BurnResult, MintArg, MintResult, SyncReceipt, Transaction,
    TransferArg, TransferResult,
};
use icrc_ledger_types::icrc1::account::Account;

#[ic_cdk::update]
pub fn icrc7_mint(arg: MintArg, caller: Principal) -> MintResult {
    // let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(crate::errors::MintError::GenericBatchError {
            error_code: 100,
            message: "Anonymous Identity".into(),
        });
    }
    STATE.with(|s| s.borrow_mut().mint(&caller, arg))
}

#[ic_cdk::update]
pub fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().icrc7_transfer(&caller, args))
}

#[ic_cdk::update]
pub fn icrc7_burn(args: Vec<BurnArg>) -> Vec<Option<BurnResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().burn(&caller, args))
}

#[ic_cdk::update]
pub fn icrc7_approve(args: Vec<ApprovalArg>) -> Vec<Option<ApproveResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().approve(&caller, args))
}

#[ic_cdk::update(guard = "owner_guard")]
pub fn icrc7_set_minting_authority(minting_account: Account) -> bool {
    STATE.with(|s| s.borrow_mut().minting_authority = Some(minting_account));
    return true;
}

#[ic_cdk::update(guard = "owner_guard")]
pub fn icrc7_set_archive_log_canister(arg: Principal) -> bool {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.archive_log_canister = Some(arg);
    });

    return true;
}

#[ic_cdk::update(guard = "owner_guard")]
pub async fn icrc7_archive_logs() -> SyncReceipt {
    let archive_log_canister = STATE
        .with(|s| s.borrow().get_archive_log_canister())
        .ok_or_else(|| InsertTransactionError::NotSetArchiveCanister)?;

    // check sync pending
    let sync_pending_txn_ids = STATE.with(|s| s.borrow().get_sync_pending_txn_ids());
    if sync_pending_txn_ids.is_some() {
        return Err(InsertTransactionError::SyncPending);
    }

    let txn_logs: Vec<Transaction> = STATE.with(|s| s.borrow().get_txn_logs(200));

    let txn_ids: Vec<u128> = txn_logs.iter().map(|log| log.txn_id).collect();

    // set pending
    STATE.with(|s| {
        s.borrow_mut()
            .set_sync_pending_txn_ids(Some(txn_ids.clone()))
    });

    // remote call logs sync
    let call_result = call_sync_logs(archive_log_canister, txn_logs).await;

    match call_result {
        Ok(count) => {
            STATE.with(|s| s.borrow_mut().remove_txn_logs(&txn_ids));
            Ok(count)
        }
        Err(_) => {
            STATE.with(|s| s.borrow_mut().set_sync_pending_txn_ids(None));
            Err(InsertTransactionError::RemoteError)
        }
    }
}
