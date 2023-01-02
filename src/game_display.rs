use crate::{common::*, logic::*};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_mod_picking::{
    DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent, Selection,
};
use core::f32::consts::PI;

pub struct GameDisplayPlugin;

impl Plugin for GameDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_startup_system(create_frame)
        .add_startup_system(make_selector)
        .add_system(replace_board)
        .add_system(pulse_selector)
        .add_system(handle_input)
        .add_system(handle_camera_movement)
        .add_system(update_player_indicator)
        .add_system(move_selector_on_click)
        .add_system(deselect_nodes);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            priority: 1,
            ..Default::default()
        },
        ..Default::default()
    });

    let font = asset_server.load("fonts/Party Confetti.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;

    commands.spawn((
        TextBundle::from_section("Hello", text_style).with_text_alignment(text_alignment),
        PlayerIndicator,
    ));

    commands.spawn((
        Camera3dBundle {
            transform: center_looking(Transform::from_xyz(-2.0, 2.5, 5.0)),
            ..default()
        },
        MainCamera,
        PickingCameraBundle::default(),
    ));
}

fn center_looking(pos: Transform) -> Transform {
    pos.looking_at(Vec3::ZERO, Vec3::Y)
}

fn move_selector_on_click(
    mut events: EventReader<PickingEvent>,
    nodes: Query<(Entity, &ClickableNode)>,
    mut selectors: Query<&mut Selector>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                for (entity, node) in &nodes {
                    if *e == entity {
                        for mut seelctor in selectors.iter_mut() {
                            seelctor.x = node.0.x;
                            seelctor.y = node.0.y;
                            seelctor.z = node.0.z;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn deselect_nodes(mut nodes: Query<&mut Selection>) {
    for mut node in nodes.iter_mut() {
        node.set_selected(false);
    }
}

#[derive(Component)]
struct MainCamera;

fn update_player_indicator(
    mut indicators: Query<&mut Text, With<PlayerIndicator>>,
    state: Res<MyGame>,
) {
    fn player_color(player: Player) -> Color {
        match player {
            Player::A => Color::WHITE,
            Player::B => Color::BLACK,
        }
    }
    let (text, color) = match state.status {
        GamePlayStatus::Playing(player) => (format!("{}", player), player_color(player)),
        GamePlayStatus::Draw => ("Draw".to_string(), Color::YELLOW),
        GamePlayStatus::Win(player) => (format!("{} won", player), player_color(player)),
    };
    for mut indicator in indicators.iter_mut() {
        indicator.sections.iter_mut().for_each(|t| {
            t.value = text.clone();
            t.style.color = color;
        });
    }
}

fn pulse_selector(time: Res<Time>, mut selectors: Query<(&mut Transform, &Selector)>) {
    let scale = (4.0 * time.elapsed().as_secs_f32()).cos() * 0.2 + 1.0;
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
    game: Res<MyGame>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    prev_markers: Query<Entity, With<Marker>>,
) {
    for marker in &prev_markers {
        commands.entity(marker).despawn();
    }
    for (z, plane) in game.board.spots.iter().enumerate() {
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

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut selectors: Query<&mut Selector>,
    mut board: ResMut<MyGame>,
    mut commands: Commands,
    selector_entity: Query<Entity, With<Selector>>,
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
            board.play(Location::new(selector.x, selector.y, selector.z));
            match board.status {
                GamePlayStatus::Playing(_) => {}
                _ => commands
                    .entity(selector_entity.single())
                    .despawn_recursive(),
            }
        }
    }
}

fn handle_camera_movement(
    input: Res<Input<KeyCode>>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    for mut camera in &mut cameras {
        if input.pressed(KeyCode::A) {
            let dir = camera.translation.normalize();
            let k = dir.cross(Vec3::Y).normalize() / 100.0 / time.delta_seconds();
            camera.translation += k;
            *camera = center_looking(*camera);
        }
        if input.pressed(KeyCode::D) {
            let dir = camera.translation.normalize();
            let k = dir.cross(Vec3::NEG_Y).normalize() / 100.0 / time.delta_seconds();
            camera.translation += k;
            *camera = center_looking(*camera);
        }

        if input.pressed(KeyCode::S) {
            let dir = camera.translation.normalize();
            let k = dir.cross(Vec3::NEG_Y).normalize();
            camera.translation += k.cross(dir) / 100.0 / time.delta_seconds();
            *camera = center_looking(*camera);
        }
        if input.pressed(KeyCode::W) {
            let dir = camera.translation.normalize();
            let k = dir.cross(Vec3::NEG_Y).normalize();
            camera.translation -= k.cross(dir) / 100.0 / time.delta_seconds();
            *camera = center_looking(*camera);
        }
        camera.translation = camera.translation.normalize() * 8.0;
    }
}

#[derive(Component)]
struct PlayerIndicator;

#[derive(Component)]
struct Marker;

#[derive(Component)]
struct Selector {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Resource, DerefMut, Deref)]
pub struct MyGame {
    pub game: TTTTState,
}

impl Default for MyGame {
    fn default() -> Self {
        Self {
            game: TTTTState {
                board: Board::new(),
                status: GamePlayStatus::Playing(Player::A),
                players: vec![Player::A, Player::B],
            },
        }
    }
}

#[derive(Component)]
struct ClickableNode(Location);

fn create_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let half_thickness = 0.01;
    let half_length = 1.5;
    let rail = meshes.add(Mesh::from(shape::Box {
        min_x: -half_thickness,
        max_x: half_thickness,
        min_y: -half_length - half_thickness,
        max_y: half_length + half_thickness,
        min_z: -half_thickness,
        max_z: half_thickness,
    }));

    for z in 0..4 {
        for y in 0..4 {
            for x in 0..4 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: 0.10,
                            ..default()
                        })),
                        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 1.0).into()),
                        transform: Transform::from_xyz(
                            x as f32 - 1.5,
                            y as f32 - 1.5,
                            z as f32 - 1.5,
                        ),
                        ..default()
                    },
                    PickableBundle::default(),
                    ClickableNode((x, y, z).into()),
                ));
            }
        }
    }

    let num_rails = 4;
    let offset = 1.5;

    for x in 0..num_rails {
        for z in 0..num_rails {
            commands.spawn(PbrBundle {
                mesh: rail.clone(),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                transform: Transform::from_xyz(x as f32 - offset, 0.0, z as f32 - offset),
                ..default()
            });
        }
    }

    for x in 0..num_rails {
        for y in 0..num_rails {
            commands.spawn(PbrBundle {
                mesh: rail.clone(),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                transform: {
                    let mut transform =
                        Transform::from_xyz(x as f32 - offset, y as f32 - offset, 0.0);
                    transform.rotate_x(PI / 2.0);
                    transform
                },
                ..default()
            });
        }
    }

    for y in 0..num_rails {
        for z in 0..num_rails {
            commands.spawn(PbrBundle {
                mesh: rail.clone(),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                transform: {
                    let mut transform =
                        Transform::from_xyz(0.0, y as f32 - offset, z as f32 - offset);
                    transform.rotate_z(PI / 2.0);
                    transform
                },
                ..default()
            });
        }
    }
}
