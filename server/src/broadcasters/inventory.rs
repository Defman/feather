//! Broadcasting of inventory-related events.

use crate::entity::EntityId;
use crate::game::Game;
use crate::network::Network;
use crate::p_inventory::{EntityInventory, Equipment, InventoryUpdateEvent};
use feather_core::inventory::{SlotIndex, SLOT_HOTBAR_OFFSET};
use feather_core::network::packet::implementation::{EntityEquipment, SetSlot};
use fecs::{Entity, World};
use num_traits::ToPrimitive;

/// System for broadcasting equipment updates.
pub fn on_inventory_update_broadcast_equipment_update(
    game: &mut Game,
    world: &mut World,
    event: &InventoryUpdateEvent,
) {
    let inv = world.get::<EntityInventory>(event.player);

    for slot in &event.slots {
        // Skip this slot if it is not an equipment update.
        if let Ok(equipment) = is_equipment_update(&inv, *slot) {
            let slot = equipment.slot_index(inv.held_item);
            let item = inv.item_at(slot).cloned();

            let packet = EntityEquipment {
                entity_id: world.get::<EntityId>(event.player).0,
                slot: equipment.to_i32().unwrap(),
                item,
            };

            game.broadcast_entity_update(world, packet, event.player, Some(event.player));
        }
    }
}

/// System to send an entity's equipment when the
/// entity is sent to a client.
pub fn on_entity_send_send_equipment(world: &mut World, entity: Entity, client: Entity) {
    if !world.is_alive(client) || !world.is_alive(entity) {
        return;
    }

    let network = world.get::<Network>(client);
    let inventory = match world.try_get::<EntityInventory>(entity) {
        Some(inv) => inv,
        None => return, // no equipment to send
    };

    let equipments = [
        Equipment::MainHand,
        Equipment::Boots,
        Equipment::Leggings,
        Equipment::Chestplate,
        Equipment::Helmet,
        Equipment::OffHand,
    ];

    for equipment in equipments.iter() {
        let item = {
            let slot = equipment.slot_index(inventory.held_item);
            inventory.item_at(slot).cloned()
        };

        let equipment_slot = equipment.to_i32().unwrap();

        let packet = EntityEquipment {
            entity_id: world.get::<EntityId>(entity).0,
            slot: equipment_slot,
            item,
        };
        network.send(packet);
    }
}

/// System for sending the Set Slot packet
/// when a player's inventory is updated.
pub fn on_inventory_update_send_set_slot(world: &mut World, event: &InventoryUpdateEvent) {
    let inv = world.get::<EntityInventory>(event.player);
    let network = world.get::<Network>(event.player);

    for slot in &event.slots {
        let packet = SetSlot {
            window_id: 0,
            slot: *slot as i16,
            slot_data: inv.item_at(*slot as usize).cloned(),
        };

        network.send(packet);
    }
}

/// Returns whether the given update to an inventory
/// is an equipment update.
fn is_equipment_update(inv: &EntityInventory, slot: SlotIndex) -> Result<Equipment, ()> {
    if slot >= SLOT_HOTBAR_OFFSET && slot - SLOT_HOTBAR_OFFSET == inv.held_item {
        Ok(Equipment::MainHand)
    } else if let Some(equipment) = Equipment::from_slot_index(slot) {
        Ok(equipment)
    } else {
        Err(())
    }
}
