use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;

#[derive(CandidType, Debug)]
pub struct InitArg {
    pub minting_authority: Option<Account>,
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Arg {
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u64>,
    pub permitted_drift: Option<u64>,
}

impl From<(Account, Arg)> for InitArg {
    fn from((account, arg): (Account, Arg)) -> Self {
        Self {
            minting_authority: Some(account),
            icrc7_symbol: arg.icrc7_symbol,
            icrc7_name: arg.icrc7_name,
            icrc7_description: arg.icrc7_description,
            icrc7_logo: arg.icrc7_logo,
            icrc7_supply_cap: arg.icrc7_supply_cap,
            icrc7_max_query_batch_size: arg.icrc7_max_query_batch_size,
            icrc7_max_update_batch_size: arg.icrc7_max_update_batch_size,
            icrc7_max_take_value: arg.icrc7_max_take_value,
            icrc7_default_take_value: arg.icrc7_default_take_value,
            icrc7_max_memo_size: arg.icrc7_max_memo_size,
            icrc7_atomic_batch_transfers: arg.icrc7_atomic_batch_transfers,
            tx_window: arg.tx_window,
            permitted_drift: arg.permitted_drift,
        }
    }
}
