use bevy::prelude::*;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<game::task::TaskChangedEvent>()
        .add_event::<game::inventory::OpenInventoryEvent>()
        .add_event::<game::input::TransformHumanEvent>()
        .add_resource(game::Owners::default())
        .add_startup_system(game::add_owners.system())
        .add_startup_system(game::item::add_items.system())
        .add_startup_system(game::inventory::add_inventories.system())
        .add_startup_system(game::dialogue::add_dialougues.system())
        .add_startup_system(game::task::add_tasks.system())
        .add_system(game::inventory::open_inventories.system())
        .add_system(game::task::tasks_changed.system())
        .add_system(game::task::tasks_in_progress.system())
        .add_system(game::input::input.system())
        .run();
}
