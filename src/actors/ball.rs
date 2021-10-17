use bevy::{input::{ElementState, mouse::*}, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct GameBall {}

pub fn setup_ball(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("actors/ball/ball.png");

    cmd.spawn_bundle(SpriteBundle {
        material: materials.add(texture.into()),
        transform: Transform {
            translation: Vec3::new(200.0, 200.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::ball(128.0),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert_bundle(RigidBodyBundle {
        position: Vec2::new(200.0, 200.0).into(),
        ..Default::default()
    })
    .insert(RigidBodyPositionSync::Discrete)
    .insert(GameBall{ });
}

pub fn ball_input(mut mouse_event: EventReader<MouseButtonInput>, mut ball: Query<(&mut RigidBodyForces, &mut RigidBodyVelocity, &RigidBodyMassProps), With<GameBall>>) {
    for e in mouse_event.iter() {
        if e.button == MouseButton::Left && e.state == ElementState::Pressed {
            println!("PRESSED omg");

            let (force, mut vel, mass) = ball.single_mut().unwrap();

            vel.apply_impulse(mass, Vec2::new(1000000.0, 1000000.0).into());
        }
    }
}