use crate::Description;
use crate::Name;
use bevy::prelude::*;

pub enum Consumable {
    GainHealth(u8),
    LooseHealth(u8),
}
pub struct Dropable;

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Item {
    Apple,
}

pub fn add_items(commands: &mut Commands) {
    commands.spawn((
        Item::Apple,
        Name("Apple"),
        Description("An tasty looking apple"),
        Dropable,
        Consumable::GainHealth(10),
    ));
}
