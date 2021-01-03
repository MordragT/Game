use crate::{Description, Name, input::OpenLogEvent};
use bevy::prelude::*;

pub struct TaskChangedEvent(Entity);

#[derive(Bundle, Debug)]
pub struct TaskBundle {
    pub task: Task,
    pub name: Name<'static>,
    pub description: Description<'static>,
    pub state: TaskState,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TaskState {
    Pending,
    Unlocked,
    InProgress,
    Completed,
    Done,
    Canceled,
}

//pub struct TaskChangedHandler(dyn Fn(TaskState));
#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Task {
    IWantToLive,
}

pub fn add_tasks(commands: &mut Commands) {
    commands.spawn(
        TaskBundle {
            task: Task::IWantToLive,
            name: Name("I want to live"),
            description: Description("I am tired and hungry, but i don't want to end up like those poor slaves. I should properly search for some food and equipment"),
            state: TaskState::InProgress,
        }    
    );
}


pub fn tasks_changed(
    mut task_changed_reader: Local<EventReader<TaskChangedEvent>>,
    tasks_changed: Res<Events<TaskChangedEvent>>,
    query: Query<(&Task, &TaskState)>,
) {
    for changed_task in task_changed_reader.iter(&tasks_changed) {
        match query.get(changed_task.0) {
            Ok((task, _state)) => {
                match task {
                    Task::IWantToLive => println!("I want to live changed!"),
                }
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

pub fn open_log(query: Query<(&'static Name, &TaskState), With<Task>>, mut open_log_reader: Local<EventReader<OpenLogEvent>>, open_log: Res<Events<OpenLogEvent>>) {
    open_log_reader.iter(&open_log).for_each(|_| {
        println!("Log:");
        query.iter().filter(|(_, state)| **state == TaskState::InProgress).for_each(|(name, _)| println!("\t{}", name));
    });
}