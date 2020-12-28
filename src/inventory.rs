use crate::item::*;
use crate::Owner;
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug)]
pub struct Inventory;

type InventoryData = HashMap<Item, u32>;

pub fn add_inventories(commands: &mut Commands) {
    let mut inventory = InventoryData::default();
    inventory.insert(Item::Apple, 1);
    commands.spawn((Inventory, Owner::Player, inventory, 1));
}

pub fn get_inventories(query: Query<&InventoryData, With<Inventory>>) {
    for inventory in query.iter() {
        dbg!(inventory);
    }
}
