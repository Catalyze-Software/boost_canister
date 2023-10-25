use candid::Principal;
use ic_cdk_macros::{post_upgrade, query, update};

use crate::{
    logic::store::{Store, E8S_PER_DAY},
    rust_declarations::types::Boosted,
};

#[post_upgrade]
pub fn post_upgrade() {
    Store::start_timers_after_upgrade();
}

#[query]
fn get_boosted_groups() -> Vec<Boosted> {
    Store::get_boosted("grp".to_string())
}

#[query]
fn get_boosted_events() -> Vec<Boosted> {
    Store::get_boosted("evt".to_string())
}

#[query]
fn get_e8s_per_day() -> u64 {
    E8S_PER_DAY
}

#[update]
async fn boost(identifier: Principal, blockheight: u64) -> Result<u64, String> {
    Store::boost(identifier, blockheight).await
}

#[query]
fn get_remaining_boost_time_in_seconds(identifier: Principal) -> u64 {
    Store::get_seconds_left_for_boosted(&identifier.to_string())
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use crate::rust_declarations::types::Boosted;
    use candid::export_service;
    use candid::Principal;
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;
    export_service!();
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().join("candid");
    write(dir.join(format!("boost.did")), __export_service()).expect("Write failed.");
}
