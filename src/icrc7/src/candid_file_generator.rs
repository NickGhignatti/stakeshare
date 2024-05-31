use candid::export_service;

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;

        let dir = env::current_dir().unwrap();
        write(dir.join("icrc7.did"), export_candid()).expect("Write failed.");
    }
}
