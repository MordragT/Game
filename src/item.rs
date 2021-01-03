use crate::{Description, GetEntity, Name};
use bevy::prelude::*;

pub enum Consumable {
    GainHealth(u8),
    LooseHealth(u8),
}
pub struct Dropable;

#[derive(Debug, Default)]
pub struct Items {
    pub apple: Option<Entity>,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Item {
    Apple,
}

impl GetEntity<Items> for Item {
    fn entity(&self, resource: &Items) -> Entity {
        match &self {
            Item::Apple => resource.apple.unwrap(),
        }
    }
}

pub fn add_items(commands: &mut Commands, mut items: ResMut<Items>) {
    commands.spawn((
        Item::Apple,
        Name("Apple"),
        Description("An tasty looking apple"),
        Dropable,
        Consumable::GainHealth(10),
    ));
    items.apple = Some(commands.current_entity().unwrap());
}
