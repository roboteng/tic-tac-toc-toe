use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_flycam::{FlyCam, PlayerPlugin};
use logic::Board;

mod logic;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .init_resource::<MyBoard>()
        .add_plugins(DefaultPlugins)
        // .add_system(setup_3d_camera)
        .add_plugin(PlayerPlugin)
        .add_system(create_board)
        .run();
}

#[derive(Resource)]
struct MyBoard {
    board: Board,
}

impl MyBoard {
    fn new(board: Board) -> Self {
        Self { board }
    }
}

impl Default for MyBoard {
    fn default() -> Self {
        Self {
            board: Board::new(),
        }
    }
}

fn setup_3d_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-8.0, 8.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.35,
        ..default()
    }));

    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let (x, y, z) = (x as f32, y as f32, z as f32);
                commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(Color::rgb(x / 3.0, y / 3.0, z / 3.0).into()),
                    transform: Transform::from_xyz(x - 1.5, y - 1.5, z - 1.5),
                    ..default()
                });
            }
        }
    }

    let rail = meshes.add(Mesh::from(shape::Box {
        min_x: -0.05,
        max_x: 0.05,
        min_y: -2.05,
        max_y: 2.05,
        min_z: -0.05,
        max_z: 0.05,
    }));

    for x in 0..5 {
        for z in 0..5 {
            commands.spawn(PbrBundle {
                mesh: rail.clone(),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                transform: Transform::from_xyz(x as f32 - 2.0, 0.0, z as f32 - 2.0),
                ..default()
            });
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            commands.spawn(PbrBundle {
                mesh: rail.clone(),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                transform: {
                    let mut transform = Transform::from_xyz(x as f32 - 2.0, y as f32 - 2.0, 0.0);
                    transform.rotate_x(PI / 2.0);
                    transform
                },
                ..default()
            });
        }
    }

    for y in 0..5 {
        for z in 0..5 {
            commands.spawn(PbrBundle {
                mesh: rail.clone(),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                transform: {
                    let mut transform = Transform::from_xyz(0.0, y as f32 - 2.0, z as f32 - 2.0);
                    transform.rotate_z(PI / 2.0);
                    transform
                },
                ..default()
            });
        }
    }
}
