#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct WalletReceiveResult {
    accepted: u64,
}

#[ic_cdk::query(name = "wallet_balance")]
pub fn wallet_balance() -> candid::Nat {
    return candid::Nat::from(ic_cdk::api::canister_balance128());
}

#[ic_cdk::update(name = "wallet_receive")]
pub fn wallet_receive() -> WalletReceiveResult {
    let available = ic_cdk::api::call::msg_cycles_available128();

    if available == 0 {
        return WalletReceiveResult { accepted: 0 };
    }
    let accepted = ic_cdk::api::call::msg_cycles_accept128(available);
    assert!(accepted == available);
    WalletReceiveResult {
        accepted: accepted as u64,
    }
}
