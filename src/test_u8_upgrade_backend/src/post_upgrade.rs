use ciborium::de;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::Memory;

use crate::{get_upgrades_memory, init_memory_manager};
use std::collections::BTreeMap;

use crate::{CanisterData, SlotDetailsV1, CANISTER_DATA};

#[post_upgrade]
fn post_upgrade() {
    restore_data_from_stable_memory();
}

fn restore_data_from_stable_memory() {
    let heap_data = get_upgrades_memory();
    let mut heap_data_len_bytes = [0; 4];
    heap_data.read(0, &mut heap_data_len_bytes);
    let heap_data_len = u32::from_le_bytes(heap_data_len_bytes) as usize;

    let mut canister_data_bytes = vec![0; heap_data_len];
    heap_data.read(4, &mut canister_data_bytes);

    let canister_data =
        de::from_reader(&*canister_data_bytes).expect("Failed to deserialize heap data");

    // let new_canister_data = convert_canister_data(&canister_data);

    CANISTER_DATA.with(|canister_data_ref_cell| {
        *canister_data_ref_cell.borrow_mut() = canister_data;
        // *canister_data_ref_cell.borrow_mut() = new_canister_data;
    });
}

// fn convert_canister_data(old_data: &CanisterData) -> CanisterData {
//     let new_test_container_field: BTreeMap<u64, SlotDetailsV1> = old_data
//         .test_container_field
//         .iter()
//         .map(|(&key, value)| {
//             (
//                 key,
//                 SlotDetailsV1 {
//                     active_room_id: value.active_room_id as u64,
//                 },
//             )
//         })
//         .collect();

//     CanisterData {
//         store_id: old_data.store_id,
//         test_container_field: new_test_container_field,
//     }
// }
