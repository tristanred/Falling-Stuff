mod arena;

use arena::walls::*;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_cameras.system())
        .add_startup_system(setup_prototype_walls.system())
        .add_system(update_walls.system())
        .run();
}
