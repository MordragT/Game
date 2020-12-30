use crate::item::*;
use crate::{Owner, Owners};
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug)]
pub struct OpenInventoryEvent(pub Owner);

type InventoryData = HashMap<Item, u32>;

#[derive(Bundle, Debug)]
pub struct InventoryBundle {
    pub data: InventoryData,
}

pub fn add_inventories(commands: &mut Commands, owners: Res<Owners>) {
    let mut player = InventoryData::default();
    player.insert(Item::Apple, 1);
    commands.insert(owners.player(), InventoryBundle { data: player });

    let mut diego = InventoryData::default();
    diego.insert(Item::Apple, 3);
    commands.insert(owners.diego(), InventoryBundle { data: diego });
}

pub fn add_item_to_inventory(data: &mut InventoryData, item: Item, amount: u32) {
    let val = match data.get(&item) {
        Some(val) => *val,
        None => 0,
    };
    data.insert(item, amount + val);
}

pub fn open_inventories(
    query: Query<&InventoryData>,
    mut inv_event_reader: Local<EventReader<OpenInventoryEvent>>,
    inv_events: Res<Events<OpenInventoryEvent>>,
    owners: Res<Owners>,
) {
    for inv_event in inv_event_reader.iter(&inv_events) {
        let owner = &inv_event.0;
        match query.get(owner.entity(&owners)) {
            Ok(data) => match owner {
                Owner::Player => println!("Player Inventory\n{:?}", data),
                _ => (),
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
