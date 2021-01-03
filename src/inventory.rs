use crate::{item::*, Description, GetEntity, Name, Owner, Owners};
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug)]
pub struct OpenInventoryEvent(pub Owner);

type InventoryData = HashMap<Item, u32>;

#[derive(Bundle, Debug)]
pub struct InventoryBundle {
    pub data: InventoryData,
}

pub fn print_inventory_data(
    data: &InventoryData,
    query: &Query<(&'static Name, &'static Description)>,
    items: &Items,
) {
    data.iter().for_each(|(item, amount)| {
        let entity = item.entity(items);
        match query.get(entity) {
            Ok((name, desc)) => println!("\t{}({}): {}", name, amount, desc),
            Err(e) => eprintln!("{:?}", e),
        }
    })
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
    query_item: Query<(&'static Name, &'static Description)>,
    mut inv_event_reader: Local<EventReader<OpenInventoryEvent>>,
    inv_events: Res<Events<OpenInventoryEvent>>,
    owners: Res<Owners>,
    items: Res<Items>,
) {
    for inv_event in inv_event_reader.iter(&inv_events) {
        let owner = &inv_event.0;
        match query.get(owner.entity(&owners)) {
            Ok(data) => match owner {
                Owner::Player => {
                    println!("Player Inventory");
                    print_inventory_data(data, &query_item, &items)
                }
                _ => (),
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
