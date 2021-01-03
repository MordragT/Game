use crate::inventory::OpenInventoryEvent;
use crate::Owner;
use bevy::{prelude::*, window::ReceivedCharacter};
#[derive(Default)]
pub struct KeyState {
    pub event_reader: EventReader<ReceivedCharacter>,
}

#[derive(Debug)]
pub enum TransformHumanEvent {
    MoveForward(Owner),
    MoveBackward(Owner),
    RotateLeft(Owner),
    RotateRight(Owner),
    Jump(Owner),
}

pub enum PlayerEvent {
    Interact,
    QuickAccess,
}

pub struct OpenLogEvent;

pub fn input(
    mut state: Local<KeyState>,
    input_events: Res<Events<ReceivedCharacter>>,
    mut open_inv: ResMut<Events<OpenInventoryEvent>>,
    mut transform_human: ResMut<Events<TransformHumanEvent>>,
    mut open_log: ResMut<Events<OpenLogEvent>>,
) {
    for event in state.event_reader.iter(&input_events) {
        match event.char {
            'i' | 'I' => open_inv.send(OpenInventoryEvent(Owner::Player)),
            'w' | 'W' => transform_human.send(TransformHumanEvent::MoveForward(Owner::Player)),
            'a' | 'A' => transform_human.send(TransformHumanEvent::RotateLeft(Owner::Player)), //look left,
            's' | 'S' => transform_human.send(TransformHumanEvent::MoveBackward(Owner::Player)), //move backwards,
            'd' | 'D' => transform_human.send(TransformHumanEvent::RotateRight(Owner::Player)), //look right,
            ' ' => transform_human.send(TransformHumanEvent::Jump(Owner::Player)), //jump
            'e' | 'E' => (),                                                       //interact
            'q' | 'Q' => (),                                                       //quickaccess
            'l' | 'L' => open_log.send(OpenLogEvent),
            i => println!("No action: {}", i),
        }
    }
}
