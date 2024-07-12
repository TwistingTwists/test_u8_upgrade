use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk_macros::export_candid;
// use ic_stable_structures::BTreeMap;
use std::borrow::Cow;
use std::cell::RefCell;
use std::{
    collections::BTreeMap,
    time::SystemTime,
};

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, Storable,
};

thread_local! {
    pub static CANISTER_DATA: RefCell<CanisterData> = RefCell::default();
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 1));

}

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
const SLOT_DETAILS_MEMORY: MemoryId = MemoryId::new(4);

type StoreId = u8;
#[derive(candid::Deserialize, CandidType, serde::Serialize, Clone, Debug)]
pub struct SlotDetailsV1 {
    pub active_room_id: StoreId,
}

#[derive(candid::Deserialize, serde::Serialize)]
pub struct CanisterData {
    pub store_id: StoreId,
    pub test_container_field: BTreeMap<u64, SlotDetailsV1>,
    // #[serde(skip, default = "_default_slot_details_map")]
    // pub store_map: ic_stable_structures::btreemap::BTreeMap<StoreId, SlotDetailsV1, Memory>,
}

pub fn _default_slot_details_map(
) -> ic_stable_structures::btreemap::BTreeMap<StoreId, SlotDetailsV1, Memory> {
    ic_stable_structures::btreemap::BTreeMap::init(get_slot_details_memory())
}
impl Default for CanisterData {
    fn default() -> Self {
        Self {
            store_id: 56,
            test_container_field: BTreeMap::new(),
            // store_map: _default_slot_details_map(),
        }
    }
}

pub fn get_slot_details_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(SLOT_DETAILS_MEMORY))
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn get_store_id() -> StoreId {
    CANISTER_DATA.with(|canister_data_ref_cell| {
        let data = canister_data_ref_cell.borrow_mut();
        data.store_id
    })
}

impl Storable for SlotDetailsV1 {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 255,
        is_fixed_size: false,
    };
}

export_candid!();
