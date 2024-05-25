use candid::Principal;
use icrc_ledger_types::icrc1::account::DEFAULT_SUBACCOUNT;

use crate::ext_types::{
    AccountIdentifier, AccountIdentifierHex, ExtApproveArg, ExtMintArg, ExtTokenIndex,
    ExtTransferArg, ExtTransferResult, TokenIdentifier,
};
use crate::guards::owner_guard;
use crate::state::STATE;

#[ic_cdk::update(name = "transfer")]
pub fn ext_transfer(arg: ExtTransferArg) -> ExtTransferResult {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().ext_transfer(&caller, arg))
}

#[ic_cdk::update(name = "approve")]
pub fn ext_approve(arg: ExtApproveArg) -> bool {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().ext_approve(&caller, arg))
}

#[ic_cdk::update(name = "mintNFT")]
pub fn ext_mint(arg: ExtMintArg) -> ExtTokenIndex {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return 0;
    }
    STATE.with(|s| s.borrow_mut().ext_mint(&caller, arg))
}

#[ic_cdk::update(name = "batchMintNFT")]
pub fn ext_batch_mint(args: Vec<ExtMintArg>) -> Vec<ExtTokenIndex> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return vec![0];
    }
    STATE.with(|s| s.borrow_mut().ext_batch_mint(&caller, args))
}

#[ic_cdk::update(name = "setAccountMapping")]
pub fn ext_set_account_mapping() -> Option<AccountIdentifierHex> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return None;
    }
    let account_id = AccountIdentifier::from_principal(&caller, &Some(*DEFAULT_SUBACCOUNT));
    STATE.with(|s| {
        s.borrow_mut()
            .ext_set_account_mapping(&caller, account_id.to_hex())
    })
}

#[ic_cdk::update(name = "updateMetadata", guard = "owner_guard")]
pub fn ext_update_metadata(token: TokenIdentifier, description: String) -> bool {
    STATE.with(|s| s.borrow_mut().ext_update_metadata(token, description))
}
