#![feature(const_fn)]

pub mod dialogue;
pub mod inventory;
pub mod item;
pub mod task;

use item::Item;
use task::{Task, TaskState};

#[derive(Debug)]
pub struct Name<'a>(pub &'a str);
#[derive(Debug)]
pub struct Description<'a>(pub &'a str);

pub enum Requirement {
    Inventory(Box<dyn Fn(&Item, &Owner) -> bool + Send + Sync>),
    Task(Box<dyn Fn(&Task, &TaskState) -> bool + Send + Sync>),
}

pub enum Effect {
    ChangeTaskState,
    GetItem,
    LooseItem,
    GetExperience,
}

pub enum Owner {
    Player,
    Diego,
}

pub mod prelude {}
