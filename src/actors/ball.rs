use bevy::prelude::*;

pub fn setup_ball(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>) {
    let texture = asset_server.load("assets/actors/ball/ball.png");
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmd.spawn_bundle(SpriteBundle {
        material: materials.add(texture.into()),
        ..Default::default()
    });

}