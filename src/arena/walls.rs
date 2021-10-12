use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection, ScalingMode};
use bevy::window::WindowResized;

enum Direction {
    LEFT,
    TOP,
    RIGHT,
    BOTTOM
}

pub struct GameWall {
    direction: Direction
}

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn update_walls(
    mut cm: Query<(&Camera, &mut OrthographicProjection)>,
    mut walls: Query<(&mut GameWall, &mut Sprite, &mut Transform)>,
    mut window_changed: EventReader<WindowResized>) {

    let (_, proj) = cm.single_mut().unwrap();

    if window_changed.iter().count() > 0 {
        walls.for_each_mut(|(w, mut s, mut t)| {
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
    }
}

pub fn setup_prototype_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>,) {
    let wall_left = create_wall_bundle(&mut materials);
    let wall_top = create_wall_bundle(&mut materials);
    let wall_right = create_wall_bundle(&mut materials);
    let wall_bottom = create_wall_bundle(&mut materials);

    commands.spawn_bundle(wall_left)
        .insert(GameWall { direction: Direction::LEFT });

    commands.spawn_bundle(wall_top)
        .insert(GameWall { direction: Direction::TOP });

    commands.spawn_bundle(wall_right)
        .insert(GameWall { direction: Direction::RIGHT });

    commands.spawn_bundle(wall_bottom)
        .insert(GameWall { direction: Direction::BOTTOM });
}

fn create_wall_bundle(materials: &mut ResMut<Assets<ColorMaterial>>) -> SpriteBundle {
    SpriteBundle {
        material: materials.add(ColorMaterial::from(Color::RED)),
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