use crate::{Description, Name};
use bevy::prelude::*;

pub struct TaskChanged {
    pub task: Task,
}

pub struct TaskBundle<'a> {
    pub task: Task,
    pub name: Name<'a>,
    pub description: Description<'a>,
    pub state: TaskState,
}

impl<'a> TaskBundle<'a> {
    fn handle(&self, f: Box<dyn Fn(&TaskState)>) {
        (f)(&self.state)
    }
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
    commands.spawn((
        TaskBundle {
            task: Task::IWantToLive,
            name: Name("I want to live"),
            description: Description("I am tired and hungry, but i don't want to end up like those poor slaves. I should properly search for some food and equipment"),
            state: TaskState::InProgress,
        },
        Box::new(|state: &TaskState| {
            match state {
                TaskState::Pending => (),
                TaskState::Unlocked => (),
                TaskState::InProgress => (),
                TaskState::Completed => println!("I want to live: completed"),
                TaskState::Done => println!("I want to live: done"),
                TaskState::Canceled => (),
            }
        })),
    );
}

pub fn listen_for_task_change(
    mut task_changed_reader: Local<EventReader<TaskChanged>>,
    tasks_changed: Res<Events<TaskChanged>>,
    query: Query<(&Box<dyn Fn(&TaskState) + Send + Sync>, &TaskState), With<Task>>,
) {
    for changed_task in task_changed_reader.iter(&tasks_changed) {
        match query.get(changed_task.task) {
            Ok((f, s)) => (f)(s),
            Err(e) => eprint!("{:?}", e),
        }
    }
}

pub fn tasks_in_progress(query: Query<(&'static Name, &TaskState), With<Task>>) {
    for task in query.iter() {
        dbg!(task);
    }
}
