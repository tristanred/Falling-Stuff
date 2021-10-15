use std::borrow::BorrowMut;

use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};
use bevy::window::{WindowCreated, WindowResized};
use bevy_rapier2d::physics::ColliderBundle;
use bevy_rapier2d::prelude::{ColliderPosition, ColliderShape, Cuboid};

enum Direction {
    LEFT,
    TOP,
    RIGHT,
    BOTTOM
}

pub struct GameWall {
    direction: Direction
}

/// No data is sent in the event because I can't tag the walls well enough
pub struct GameWallSizeChanged { }

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn update_walls(
    mut cm: Query<(&Camera, &mut OrthographicProjection)>,
    mut walls: Query<(&mut GameWall, &mut Sprite, &mut Transform, Option<&mut ColliderShape>)>,
    mut window_created: EventReader<WindowCreated>,
    mut window_resized: EventReader<WindowResized>,
    mut wall_resized: EventWriter<GameWallSizeChanged>) {

    let count_created = window_created.iter().count();
    let count_resized = window_resized.iter().count();

    if count_created == 0 && count_resized == 0 {
        return;
    }

    println!("Doing stuff");

    let (_, proj) = cm.single_mut().unwrap();

    walls.for_each_mut(|(w, mut s, mut t, c)| {
        match w.direction {
            Direction::LEFT => {
                // Set the size to be as high as the screen. Keep the
                // width to 20.
                let height = proj.top + proj.bottom.abs();
                let width = 20.0;

                s.size.x = width;
                s.size.y = height;

                // Set the position to the left edge
                t.translation.x = proj.left;
                t.translation.y = 0.0;
            },
            Direction::TOP => {
                // Set the size to be as wide as the screen. Keep the
                // height to 20.
                let height = 20.0;
                let width = proj.left.abs() + proj.right.abs();

                s.size.x = width;
                s.size.y = height;

                // Set the position to the top edge
                t.translation.x = 0.0;
                t.translation.y = proj.top;
            },
            Direction::RIGHT => {
                // Set the size to be as high as the screen. Keep the
                // width to 20.
                let height = proj.top + proj.bottom.abs();
                let width = 20.0;

                s.size.x = width;
                s.size.y = height;

                // Set the position to the right edge
                t.translation.x = proj.right;
                t.translation.y = 0.0;
            },
            Direction::BOTTOM => {
                // Set the size to be as wide as the screen. Keep the
                // height to 20.
                let height = 20.0;
                let width = proj.left.abs() + proj.right.abs();

                s.size.x = width;
                s.size.y = height;

                // Set the position to the bottom edge
                t.translation.x = 0.0;
                t.translation.y = proj.bottom;
            },
        }
    });

    wall_resized.send(GameWallSizeChanged{ });
}

pub fn setup_prototype_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>,) {
    let wall_left = create_wall_bundle(materials.add(ColorMaterial::from(Color::RED)));
    let wall_top = create_wall_bundle(materials.add(ColorMaterial::from(Color::BLUE)));
    let wall_right = create_wall_bundle(materials.add(ColorMaterial::from(Color::GREEN)));
    let wall_bottom = create_wall_bundle(materials.add(ColorMaterial::from(Color::YELLOW)));

    commands.spawn_bundle(wall_left)
        .insert(GameWall { direction: Direction::LEFT })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(100.0, 100.0),
            ..Default::default()
        });

    commands.spawn_bundle(wall_top)
        .insert(GameWall { direction: Direction::TOP })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(100.0, 100.0),
            ..Default::default()
        });

    commands.spawn_bundle(wall_right)
        .insert(GameWall { direction: Direction::RIGHT })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(100.0, 100.0),
            ..Default::default()
        });

    commands.spawn_bundle(wall_bottom)
        .insert(GameWall { direction: Direction::BOTTOM })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(100.0, 100.0),
            ..Default::default()
        });
}

pub fn update_wall_colliders(
    mut commands: Commands,
    mut walls: Query<(Entity, &Sprite), With<GameWall>>,
    mut wall_changed_event: EventReader<GameWallSizeChanged>
) {
    if wall_changed_event.iter().count() > 0 {
        for (e, s) in walls.iter_mut() {
            commands
                .entity(e)
                .remove_bundle::<ColliderBundle>()
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(s.size.x / 2.0, s.size.y / 2.0),
                    ..Default::default()
                });
        }
    }
}

/// Create a SpriteBundle containing the materials, size and transforms
/// necessary for a Wall entity.
fn create_wall_bundle(color: Handle<ColorMaterial>) -> SpriteBundle {
    SpriteBundle {
        material: color,
        sprite: Sprite {
          size: Vec2::new(100.0, 100.0),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    }
}