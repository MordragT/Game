use bevy::prelude::*;
fn main() {
    App::build()
        .add_event::<game::task::TaskChanged>()
        .add_startup_system(game::item::add_items.system())
        .add_startup_system(game::inventory::add_inventories.system())
        .add_startup_system(game::dialogue::add_dialougues.system())
        .add_startup_system(game::task::add_tasks.system())
        .add_system(game::inventory::get_inventories.system())
        .run();
}
