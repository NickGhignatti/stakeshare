use candid::Principal;
use ic_cdk::caller;
use icrc_ledger_types::icrc1::account::Account;

use crate::{icrc7_types::Transaction, state::STATE, Icrc7TokenMetadata, Standard};

#[ic_cdk::query]
pub fn whoami() -> String {
    ic_cdk::println!("Caller of ICRC7 canister = {}", caller());
    caller().to_string()
}

#[ic_cdk::query]
pub fn icrc7_symbol() -> String {
    STATE.with(|s| s.borrow().icrc7_symbol())
}

#[ic_cdk::query]
pub fn icrc7_name() -> String {
    STATE.with(|s| s.borrow().icrc7_name())
}

#[ic_cdk::query]
pub fn icrc7_description() -> Option<String> {
    STATE.with(|s| s.borrow().icrc7_description())
}

#[ic_cdk::query]
pub fn icrc7_logo() -> Option<String> {
    STATE.with(|s| s.borrow().icrc7_logo())
}

#[ic_cdk::query]
pub fn icrc7_total_supply() -> u128 {
    STATE.with(|s| s.borrow().icrc7_total_supply())
}

#[ic_cdk::query]
pub fn icrc7_supply_cap() -> Option<u128> {
    STATE.with(|s| s.borrow().icrc7_supply_cap())
}

#[ic_cdk::query]
pub fn icrc7_minting_authority() -> Option<Account> {
    STATE.with(|s| s.borrow().icrc7_minting_authority())
}

#[ic_cdk::query]
pub fn icrc7_max_query_batch_size() -> Option<u128> {
    STATE.with(|s| s.borrow().icrc7_max_query_batch_size())
}

#[ic_cdk::query]
pub fn icrc7_max_update_batch_size() -> Option<u128> {
    STATE.with(|s| s.borrow().icrc7_max_update_batch_size())
}

#[ic_cdk::query]
pub fn icrc7_default_take_value() -> Option<u128> {
    STATE.with(|s| s.borrow().icrc7_default_take_value())
}

#[ic_cdk::query]
pub fn icrc7_max_take_value() -> Option<u128> {
    STATE.with(|s| s.borrow().icrc7_max_take_value())
}

#[ic_cdk::query]
pub fn icrc7_max_memo_size() -> Option<u128> {
    STATE.with(|s| s.borrow().icrc7_max_memo_size())
}

#[ic_cdk::query]
pub fn icrc7_atomic_batch_transfers() -> Option<bool> {
    STATE.with(|s| s.borrow().icrc7_atomic_batch_transfers())
}

#[ic_cdk::query]
pub fn icrc7_owner_of(ids: Vec<u128>) -> Vec<Option<Account>> {
    STATE.with(|s| s.borrow().icrc7_owner_of(&ids))
}

#[ic_cdk::query]
pub fn icrc7_supported_standards() -> Vec<Standard> {
    vec![Standard {
        name: "ICRC-7".into(),
        url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-7".into(),
    }]
}

#[ic_cdk::query]
pub fn icrc7_archive_log_canister() -> Option<Principal> {
    STATE.with(|s| s.borrow().get_archive_log_canister())
}

#[ic_cdk::query]
pub fn icrc7_tokens(prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    STATE.with(|s| s.borrow().icrc7_tokens(prev, take))
}

#[ic_cdk::query]
pub fn icrc7_token_metadata(token_ids: Vec<u128>) -> Vec<Option<Icrc7TokenMetadata>> {
    STATE.with(|s| s.borrow().icrc7_token_metadata(&token_ids))
}

#[ic_cdk::query]
pub fn icrc7_balance_of(accounts: Vec<Account>) -> Vec<u128> {
    STATE.with(|s| s.borrow().icrc7_balance_of(&accounts))
}

#[ic_cdk::query]
pub fn icrc7_tokens_of(account: Account, prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    STATE.with(|s| s.borrow().icrc7_tokens_of(account, prev, take))
}

#[ic_cdk::query]
pub fn icrc7_txn_logs(page_number: u32, page_size: u32) -> Vec<Transaction> {
    STATE.with(|s| s.borrow().icrc7_txn_logs(page_number, page_size))
}
