use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_system(setup_3d_camera)
        .add_system(create_spheres)
        .run();
}

fn setup_3d_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-8.0, 8.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn create_spheres(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 0.7 }));

    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let (x, y, z) = (x as f32, y as f32, z as f32);
                commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(Color::rgb(x / 3.0, y / 3.0, z / 3.0).into()),
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                });
            }
        }
    }
}
