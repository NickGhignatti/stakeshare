use std::str::FromStr;

use candid::Principal;
use ic_cdk::caller;

// convert a string slice to a principal
fn slice_to_principal(principal: &str) -> Principal {
    match Principal::from_str(principal) {
        Ok(p) => p,
        _ => Principal::anonymous(),
    }
}

pub fn not_anonymous_caller() -> Result<(), String> {
    let caller = caller();

    let backend_principal = slice_to_principal(
        #[allow(clippy::option_env_unwrap)]
        option_env!("CANISTER_ID_ICRC7_BACKEND")
            .expect("Env variable CANISTER_ID_FACTORY not found!"),
    );

    if caller != backend_principal {
        return Err("Caller is not the backend of StakeShare dapp!.".to_string());
    }
    Ok(())
}
