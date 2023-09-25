use std::cell::RefCell;

use ic_ledger_types::Tokens;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    {DefaultMemoryImpl, StableBTreeMap, StableCell},
};

use crate::rust_declarations::types::Boosted;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static E8S_PER_DAY: u64 = 10000;

    pub static BOOSTED: RefCell<StableBTreeMap<String, Boosted, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );

    pub static LAST_BLOCK_HEIGHT: RefCell<StableCell<u64, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
            0
        ).expect("failed")
    );
}

pub struct Store {}

impl Store {
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
}
