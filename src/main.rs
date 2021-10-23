mod arena;
mod actors;

use arena::walls::*;
use bevy::{prelude::*, render::{RenderSystem, camera::{self, OrthographicProjection}}};
use actors::ball::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin, widgets::InspectorQuerySingle};
use bevy_rapier2d::{physics::{NoUserData, RapierPhysicsPlugin}, prelude::ColliderPosition, render::RapierRenderPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup_cameras.system())
        // Set game Walls
        .add_startup_system(setup_prototype_walls.system())
        //.add_system_to_stage(CoreStage::PostUpdate, update_walls.system().after(RenderSystem::VisibleEntities))
        //.add_system(update_walls.system())
        .add_system(update_wall_colliders.system())
        .add_event::<GameWallSizeChanged>()
        // Set bouncing ball
        .add_startup_system(setup_ball.system())
        .add_system(ball_input.system())
        .run();
}
