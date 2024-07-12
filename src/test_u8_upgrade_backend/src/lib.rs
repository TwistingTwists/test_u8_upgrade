use candid::{CandidType, Decode, Encode};
// use ic_cdk::{post_upgrade, pre_upgrade, storage};
use ic_cdk_macros::export_candid;
// use ic_stable_structures::BTreeMap;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, Storable,
};
pub mod post_upgrade;
pub mod pre_upgrade;
thread_local! {
    pub static CANISTER_DATA: RefCell<CanisterData> = RefCell::default();
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 1));

}

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
const SLOT_DETAILS_MEMORY: MemoryId = MemoryId::new(4);
const UPGRADES: MemoryId = MemoryId::new(0);

type StoreId = u8;
#[derive(candid::Deserialize, CandidType, serde::Serialize, Clone, Debug)]
pub struct SlotDetailsV1 {
    pub active_room_id: StoreId,
}

#[derive(candid::Deserialize, CandidType, serde::Serialize, Clone, Debug)]
#[serde(from = "SlotDetailsV1")]
pub struct SlotDetailsV2 {
    pub active_room_id: u64,
}

impl From<SlotDetailsV1> for SlotDetailsV2 {
    fn from(value: SlotDetailsV1) -> Self {
        Self {
            active_room_id: value.active_room_id as u64,
        }
    }
}

#[derive(candid::Deserialize, CandidType, serde::Serialize)]
pub struct CanisterData {
    pub store_id: StoreId,
    pub test_container_field: BTreeMap<u64, SlotDetailsV1>,
    // #[serde(alias = "test_container_field")]
    // pub test_container_field_1: BTreeMap<u64, SlotDetailsV2>,
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
            test_container_field: default_vals_v1(),
            // test_container_field_1: default_vals_v2(),
            // test_container_field_1: BTreeMap::new(),
            // store_map: _default_slot_details_map(),
        }
    }
}
pub fn default_vals_v1() -> BTreeMap<u64, SlotDetailsV1> {
    let mut slot_map: BTreeMap<u64, SlotDetailsV1> = BTreeMap::new();

    // Adding some sample values
    slot_map.insert(1, SlotDetailsV1 { active_room_id: 5 });
    slot_map.insert(3, SlotDetailsV1 { active_room_id: 2 });
    slot_map.insert(7, SlotDetailsV1 { active_room_id: 8 });
    slot_map.insert(255, SlotDetailsV1 { active_room_id: 1 });
    slot_map
}

pub fn default_vals_v2() -> BTreeMap<u64, SlotDetailsV2> {
    let mut slot_map: BTreeMap<u64, SlotDetailsV2> = BTreeMap::new();

    // Adding some sample values
    slot_map.insert(1, SlotDetailsV2 { active_room_id: 5 });
    slot_map.insert(3, SlotDetailsV2 { active_room_id: 2 });
    slot_map.insert(7, SlotDetailsV2 { active_room_id: 8 });
    slot_map.insert(255, SlotDetailsV2 { active_room_id: 1 });
    // slot_map.insert(255, SlotDetailsV1 { active_room_id: 1 });
    slot_map
}

pub fn init_memory_manager() {
    MEMORY_MANAGER.with(|m| {
        *m.borrow_mut() = MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 1);
    })
}

pub fn get_slot_details_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(SLOT_DETAILS_MEMORY))
}

pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow_mut().get(UPGRADES))
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

#[ic_cdk_macros::query]
fn get_active_room_id_v1() -> BTreeMap<u64, SlotDetailsV1> {
    CANISTER_DATA.with(|canister_data_ref_cell| {
        let data = canister_data_ref_cell.borrow_mut();
        data.test_container_field.clone()
    })
}

// #[ic_cdk_macros::query]
// fn get_active_room_id_v2() -> BTreeMap<u64, SlotDetailsV2> {
//     CANISTER_DATA.with(|canister_data_ref_cell| {
//         let data = canister_data_ref_cell.borrow_mut();
//         data.test_container_field_1.clone()
//     })
// }

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
