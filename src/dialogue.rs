use crate::task::{Task, TaskState};
use crate::{Owner, Requirement};
use bevy::prelude::*;

pub struct DialogueOptions<'a>(pub Vec<&'a str>);

pub enum DialogueState {
    Active,
    Inactive,
}

#[derive(Debug)]
pub struct Dialogue;

pub fn add_dialougues(commands: &mut Commands) {
    commands.spawn((
        Dialogue,
        Owner::Diego,
        // Requirement::Task(Box::new(|task, state| {
        //     *task == Task::IWantToLive && *state == TaskState::InProgress
        // })),
        DialogueState::Active,
        DialogueOptions(vec![
            "Where am i ?",
            "Who are you ?",
            "Where can i find work ?",
        ]),
    ));
}
