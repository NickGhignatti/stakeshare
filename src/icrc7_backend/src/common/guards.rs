use candid::Principal;
use ic_cdk::caller;

// generic guard to check that the caller is not anonymous
pub fn not_anonymous_caller() -> Result<(), String> {
    let caller = caller();
    if caller == Principal::anonymous() {
        return Err("Caller is anonymous.".to_string());
    }
    Ok(())
}
