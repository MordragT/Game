use crate::Owners;
use bevy::prelude::*;

// pub struct DialogueBuilder {
//     owner: Owner,
//     init: DialogueState,
//     current: Option<&'static str>,
// }

// impl DialogueBuilder {
//     pub fn new(owner: Owner, init: DialogueState) -> Self {
//         Self {
//             owner,
//             init,
//             current: None,
//         }
//     }
//     pub fn append(&mut self, child: &'static str) -> &mut Self {
//         self.current = Some(child);
//         &mut self
//     }
//     pub fn spawn(self, commands: &mut Commands) -> Result<(), &str> {
//         if self.current.is_none() {
//             return Err("No root specified");
//         }
//         Ok(())
//     }
// }

// #[derive(Bundle)]
// pub struct DialogueBundle<const N: usize> {
//     owner: Owner,
//     option: &'static str,
//     pub state: DialogueState,
//     children: [Entity; N],
// }

// impl<const N: usize> DialogueBundle<N> {
//     pub fn new(owner: Owner, option: &'static str, children: [Entity; N]) -> Self {
//         Self {
//             owner,
//             option,
//             state: DialogueState::Inactive,
//             children,
//         }
//     }
// }

// pub struct DialogueOption<'a> {
//     inner: &'a str,
//     next: Option<Entity>,
// }

// impl<'a> DialogueOption<'a> {
//     pub fn new(inner: &'a str, next: Option<Entity>) -> Self {
//         Self { inner, next }
//     }
// }
pub enum DialogueState {
    Inactive,
    Active,
    Triggered,
}

#[derive(Bundle)]
pub struct DialogueBundle {
    pub option: &'static str,
    pub state: DialogueState,
}

pub struct DialougueTriggered(Entity);

pub fn add_dialougues(commands: &mut Commands, owners: Res<Owners>) {
    /*
           Where am I ?
                |
        Is the castle far ?
        |                   |
    Where can i       Can u help me get safely to the city ?
    get a weapon ?
        |
    I never swong a sword.


    */
    // let builder = DialogueBuilder::new(Owner::Diego, DialogueState::Active)
    //     .append("Where am I ?")
    //     .append("Is the castle far ?");
    // builder
    //     .append("Where can i get a weapon ?")
    //     .append("I never swong a sword.");
    // builder.append("Can u help me get safely to the city ?");
    // builder.spawn(commands);

    // commands.spawn(DialogueBundle::new(
    //     Owner::Diego,
    //     "Where am I?",
    //     [commands.spawn(DialogueBundle::new(Owner::Diego, "Is the castle far?", []))],
    // ));
    commands.insert(
        owners.diego(),
        DialogueBundle {
            option: "Where am I ?",
            state: DialogueState::Active,
        },
    );
    commands.insert(
        owners.diego(),
        DialogueBundle {
            option: "Can you train me?",
            state: DialogueState::Active,
        },
    );
}
