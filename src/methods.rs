use std::time::Duration;

use candid::Principal;
use ic_cdk::{api::time, caller};
use ic_cdk_macros::{query, update};
use ic_cdk_timers::set_timer;
use ic_scalable_misc::models::identifier_model::Identifier;

use crate::{
    logic::{
        ledger::Ledger,
        store::{Store, BOOSTED},
    },
    rust_declarations::types::Boosted,
};

#[query]
fn get_boosted_by_identifier(identifier: Principal) -> Option<Boosted> {
    BOOSTED.with(|p| {
        p.borrow()
            .iter()
            .find(|(key, _)| key == &identifier.to_string())
            .map(|(_, v)| v)
    })
}

#[update]
async fn boost(identifier: Principal, blockheight: u64) -> Option<u64> {
    let (_, _, kind) = Identifier::decode(&identifier);

    match Ledger::validate_transaction(caller(), blockheight).await {
        Some(amount) => {
            let days = Store::calculate_days(amount);
            let boosted = Boosted {
                identifier,
                days,
                created_at: time(),
                blockheight,
                owner: caller(),
                type_: kind,
            };

            if let Some(mut existing) = BOOSTED.with(|b| b.borrow().get(&identifier.to_string())) {
                existing.days += days;
                BOOSTED.with(|p| {
                    p.borrow_mut()
                        .insert(identifier.to_string(), existing.clone())
                });
                set_timer(
                    Duration::from_secs(Store::get_seconds_from_days(existing.days)),
                    move || remove_boost(identifier),
                );
                return Some(existing.days);
            } else {
                // Insert boost into the boosted list
                BOOSTED.with(|p| {
                    p.borrow_mut()
                        .insert(identifier.to_string(), boosted.clone())
                });

                set_timer(
                    Duration::from_secs(Store::get_seconds_from_days(days)),
                    move || remove_boost(identifier),
                );
                return Some(days);
            }
        }
        None => None,
    }
}

#[update]
fn remove_boost(identifier: Principal) {
    BOOSTED.with(|p| p.borrow_mut().remove(&identifier.to_string()));
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
