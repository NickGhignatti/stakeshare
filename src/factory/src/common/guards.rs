use candid::Principal;
use ic_cdk::caller;

pub fn not_anonymous_caller() -> Result<(), String> {
    let caller = caller();
    if caller == Principal::anonymous() {
        return Err("Caller is anonymous.".to_string());
    }
    Ok(())
}
