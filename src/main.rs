mod arena;
mod actors;

use arena::walls::*;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup_cameras.system())
        // Set game Walls
        .add_startup_system(setup_prototype_walls.system())
        .add_system(update_walls.system())
        .add_system(update_wall_colliders.system())
        .add_event::<GameWallSizeChanged>()
        // Set bouncing ball
        .run();
}
