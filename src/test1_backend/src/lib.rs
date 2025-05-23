#![allow(unused_imports)]
#![allow(dead_code)]

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use serde::Deserialize;

type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType, Deserialize, Clone)]
struct Exam {
    course: String,
    out_of: u32,
    score: u32,
}

impl Storable for Exam {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Exam {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static EXAM_MAP: RefCell<StableBTreeMap<u64, Exam, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );
}

#[ic_cdk::query]
fn get_participation(key: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
fn get_exam(key: u64) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::update]
fn insert_exam(key: u64, value: Exam) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow_mut().insert(key, value))
}

#[ic_cdk::update]
fn insert_participation(key: u64, value: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow_mut().insert(key, value))
}