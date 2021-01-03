#![feature(const_fn)]
#![feature(min_const_generics)]

pub mod dialogue;
pub mod input;
pub mod inventory;
pub mod item;
pub mod task;

use bevy::prelude::*;
use std::fmt;
// use item::Item;
// use task::{Task, TaskState};

#[derive(Debug)]
pub struct Name<'a>(pub &'a str);

impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
#[derive(Debug)]
pub struct Description<'a>(pub &'a str);

impl<'a> fmt::Display for Description<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

// pub enum Requirement {
//     Inventory(Box<dyn Fn(&Item, &Owner) -> bool + Send + Sync>),
//     Task(Box<dyn Fn(&Task, &TaskState) -> bool + Send + Sync>),
// }

// pub enum Effect {
//     ChangeTaskState,
//     GetItem,
//     LooseItem,
//     GetExperience,
// }

pub trait GetEntity<R> {
    fn entity(&self, resource: &R) -> Entity;
}

#[derive(Debug, Default)]
pub struct Owners {
    pub player: Option<Entity>,
    pub diego: Option<Entity>,
}

impl Owners {
    pub fn player(&self) -> Entity {
        self.player.unwrap()
    }
    pub fn diego(&self) -> Entity {
        self.diego.unwrap()
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Owner {
    Player,
    Diego,
}

impl GetEntity<Owners> for Owner {
    fn entity(&self, resource: &Owners) -> Entity {
        match &self {
            Owner::Player => resource.player(),
            Owner::Diego => resource.diego(),
        }
    }
}

#[derive(Bundle)]
pub struct OwnerBundle {
    pub owner: Owner,
}

pub fn add_owners(commands: &mut Commands, mut owners: ResMut<Owners>) {
    commands.spawn(OwnerBundle {
        owner: Owner::Diego,
    });
    owners.diego = Some(commands.current_entity().unwrap());

    commands.spawn(OwnerBundle {
        owner: Owner::Player,
    });
    owners.player = Some(commands.current_entity().unwrap());
}

pub mod prelude {}
