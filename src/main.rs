use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use logic::{Board, Player};

mod logic;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource({
            let mut b = Board::new();
            b.place(Player::B, 0, 2, 3).unwrap();
            b.place(Player::A, 0, 0, 0).unwrap();
            b.place(Player::A, 3, 3, 3).unwrap();
            MyBoard { board: b }
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(create_frame)
        .add_startup_system(make_selector)
        .add_system(replace_board)
        .add_system(pulse_selector)
        .add_system(handle_input)
        .run();
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut selectors: Query<&mut Selector>,
    mut board: ResMut<MyBoard>,
) {
    if input.just_pressed(KeyCode::I) {
        for mut selector in selectors.iter_mut() {
            selector.y = (selector.y + 1) % 4
        }
    }
    if input.just_pressed(KeyCode::K) {
        for mut selector in selectors.iter_mut() {
            selector.y = (4 + selector.y - 1) % 4
        }
    }
    if input.just_pressed(KeyCode::J) {
        for mut selector in selectors.iter_mut() {
            selector.x = (selector.x + 1) % 4
        }
    }
    if input.just_pressed(KeyCode::L) {
        for mut selector in selectors.iter_mut() {
            selector.x = (4 + selector.x - 1) % 4
        }
    }
    if input.just_pressed(KeyCode::U) {
        for mut selector in selectors.iter_mut() {
            selector.z = (selector.z + 1) % 4
        }
    }
    if input.just_pressed(KeyCode::O) {
        for mut selector in selectors.iter_mut() {
            selector.z = (4 + selector.z - 1) % 4
        }
    }
    if input.just_pressed(KeyCode::Return) {
        for selector in &selectors {
            board
                .place(Player::A, selector.x, selector.y, selector.z)
                .unwrap();
        }
    }
}

fn pulse_selector(time: Res<Time>, mut selectors: Query<(&mut Transform, &Selector)>) {
    let scale = time.elapsed().as_secs_f32().cos() * 0.2 + 1.0;
    for (mut transfrom, selector) in selectors.iter_mut() {
        transfrom.scale = Vec3::splat(scale);
        transfrom.translation = Transform::from_xyz(
            selector.x as f32 - 1.5,
            selector.y as f32 - 1.5,
            selector.z as f32 - 1.5,
        )
        .translation;
    }
}

fn make_selector(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.35,
                ..default()
            })),
            material: materials.add(Color::rgba(0.0, 1.0, 0.0, 0.5).into()),
            transform: Transform::from_xyz(1.0 - 1.5, 2.0 - 1.5, 3.0 - 1.5),
            ..default()
        })
        .insert(Selector { x: 1, y: 2, z: 3 });
}

fn replace_board(
    mut commands: Commands,
    board: Res<MyBoard>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    prev_markers: Query<Entity, With<Marker>>,
) {
    for marker in &prev_markers {
        commands.entity(marker).despawn();
    }
    for (z, plane) in board.spots.iter().enumerate() {
        for (y, row) in plane.iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                if let Some(player) = pos {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: 0.35,
                                ..default()
                            })),
                            material: materials.add(
                                (if *player == Player::A {
                                    Color::rgba(1.0, 1.0, 1.0, 0.75)
                                } else {
                                    Color::rgba(0.0, 0.0, 0.0, 0.75)
                                })
                                .into(),
                            ),
                            transform: Transform::from_xyz(
                                x as f32 - 1.5,
                                y as f32 - 1.5,
                                z as f32 - 1.5,
                            ),
                            ..default()
                        })
                        .insert(Marker);
                }
            }
        }
    }
}

#[derive(Component)]
struct Marker;

#[derive(Component)]
struct Selector {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Resource, DerefMut, Deref)]
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

fn create_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
