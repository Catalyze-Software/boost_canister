use std::{cell::RefCell, collections::HashMap, time::Duration};

use candid::Principal;
use ic_cdk::{api::time, caller};
use ic_cdk_timers::{clear_timer, set_timer, TimerId};
use ic_ledger_types::Tokens;
use ic_scalable_misc::models::identifier_model::Identifier;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    {DefaultMemoryImpl, StableBTreeMap, StableCell},
};

use crate::rust_declarations::types::Boosted;

use super::ledger::Ledger;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static E8S_PER_DAY: u64 = 10000;

    pub static BOOSTED: RefCell<StableBTreeMap<String, Boosted, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
        )
    );

    pub static LAST_BLOCK_HEIGHT: RefCell<StableCell<u64, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
            0
        ).expect("failed")
    );

    pub static TIMERS: RefCell<HashMap<String, TimerId>> = RefCell::new(HashMap::default());
}

pub struct Store {}

impl Store {
    pub async fn boost(identifier: Principal, blockheight: u64) -> Result<u64, String> {
        let (_, _, kind) = Identifier::decode(&identifier);

        match Ledger::validate_transaction(caller(), blockheight).await {
            Some(amount) => {
                if blockheight > Store::get_last_block_height() {
                    Store::set_last_block_height(blockheight);
                } else {
                    return Err("Blockheight is lower than the last blockheight".to_string());
                }

                let days = Store::calculate_days(amount);
                let seconds = Store::get_seconds_from_days(days);

                if let Some(mut existing) =
                    BOOSTED.with(|b| b.borrow().get(&identifier.to_string()))
                {
                    if let Some(existing_timer_id) = Store::get_timer_id(identifier.to_string()) {
                        clear_timer(existing_timer_id);
                    }

                    let remaining_seconds =
                        Store::get_seconds_left_for_boosted(&identifier.to_string());
                    existing.seconds = remaining_seconds + seconds;
                    existing.updated_at = time();

                    BOOSTED.with(|p| {
                        p.borrow_mut()
                            .insert(identifier.to_string(), existing.clone())
                    });

                    let timer_id = set_timer(Duration::from_secs(existing.seconds), move || {
                        Store::remove_boost(&identifier.to_string())
                    });

                    Store::set_timer_id(identifier.to_string(), timer_id);
                    return Ok(existing.seconds);
                } else {
                    let boosted = Boosted {
                        identifier,
                        seconds,
                        created_at: time(),
                        updated_at: time(),
                        blockheight,
                        owner: caller(),
                        type_: kind,
                    };

                    let timer_id = set_timer(Duration::from_secs(seconds), move || {
                        Store::remove_boost(&identifier.to_string())
                    });

                    Store::set_timer_id(identifier.to_string(), timer_id);

                    // Insert boost into the boosted list
                    BOOSTED.with(|p| {
                        p.borrow_mut()
                            .insert(identifier.to_string(), boosted.clone())
                    });

                    return Ok(seconds);
                }
            }
            None => Err("No transaction found".to_string()),
        }
    }

    pub async fn test_boost(identifier: Principal, seconds: u64) -> Result<u64, String> {
        let (_, _, kind) = Identifier::decode(&identifier);

        if let Some(mut existing) = BOOSTED.with(|b| b.borrow().get(&identifier.to_string())) {
            if let Some(existing_timer_id) = Store::get_timer_id(identifier.to_string()) {
                clear_timer(existing_timer_id);
            }

            let remaining_seconds = Store::get_seconds_left_for_boosted(&identifier.to_string());
            existing.seconds = remaining_seconds + seconds;
            existing.updated_at = time();

            let timer_id = set_timer(Duration::from_secs(existing.seconds), move || {
                Store::remove_boost(&identifier.to_string())
            });

            Store::set_timer_id(identifier.to_string(), timer_id);

            BOOSTED.with(|p| {
                p.borrow_mut()
                    .insert(identifier.to_string(), existing.clone())
            });
            return Ok(existing.seconds);
        } else {
            let boosted = Boosted {
                identifier,
                seconds,
                created_at: time(),
                updated_at: time(),
                blockheight: 0,
                owner: caller(),
                type_: kind,
            };

            // Insert boost into the boosted list
            BOOSTED.with(|p| {
                p.borrow_mut()
                    .insert(identifier.to_string(), boosted.clone())
            });

            let timer_id = set_timer(Duration::from_secs(seconds), move || {
                Store::remove_boost(&identifier.to_string())
            });

            Store::set_timer_id(identifier.to_string(), timer_id);
            return Ok(seconds);
        }
    }

    pub fn remove_boost(identifier: &String) {
        BOOSTED.with(|p| p.borrow_mut().remove(&identifier));
        Store::remove_timer_id(identifier.to_string());
    }

    pub fn calculate_days(tokens: Tokens) -> u64 {
        let e8s_per_day = E8S_PER_DAY.with(|e| *e);
        let days = ((tokens.e8s() as f64) / (e8s_per_day as f64)).round() as u64;
        days
    }

    pub fn get_seconds_from_days(days: u64) -> u64 {
        days * 24 * 60 * 60
    }

    pub fn set_last_block_height(block_height: u64) {
        LAST_BLOCK_HEIGHT.with(|b| {
            let _ = b.borrow_mut().set(block_height);
        });
    }

    pub fn get_last_block_height() -> u64 {
        LAST_BLOCK_HEIGHT.with(|b| b.borrow().get().clone())
    }

    pub fn get_boosted(kind: String) -> Vec<Boosted> {
        BOOSTED.with(|p| {
            p.borrow()
                .iter()
                .filter(|v| v.1.type_ == kind.to_string())
                .map(|v| v.1.clone())
                .collect()
        })
    }

    pub fn set_timer_id(identifier: String, timer_id: TimerId) {
        TIMERS.with(|t| {
            t.borrow_mut().insert(identifier, timer_id);
        });
    }

    pub fn get_timer_id(identifier: String) -> Option<TimerId> {
        TIMERS.with(|t| t.borrow().get(&identifier).cloned())
    }

    pub fn remove_timer_id(identifier: String) {
        TIMERS.with(|t| {
            t.borrow_mut().remove(&identifier);
        });
    }

    pub fn get_seconds_left_for_boosted(identifier: &String) -> u64 {
        BOOSTED
            .with(|b| {
                b.borrow().get(identifier).map(|b| {
                    let time_left: u64 = Duration::from_nanos(b.updated_at).as_secs() + b.seconds;
                    time_left - Duration::from_nanos(time()).as_secs()
                })
            })
            .unwrap_or(0)
    }

    pub fn start_timers_after_upgrade() {
        BOOSTED.with(|b| {
            b.borrow().iter().for_each(|(identifier, _)| {
                let _identifier = identifier.clone();
                let timer_id = set_timer(
                    Duration::from_secs(Store::get_seconds_left_for_boosted(&identifier)),
                    move || Store::remove_boost(&identifier),
                );

                Store::set_timer_id(_identifier, timer_id);
            })
        });
    }
}
