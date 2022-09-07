use bevy::prelude::*;

#[derive(Component)]
struct Cube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(cube_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Make a cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    // Custom components
    .insert(Cube);

    // Make a camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn cube_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, With<Cube>)>
){
    for (mut cube_transform, _) in query.iter_mut() {
        let pos = &mut cube_transform.translation;
        
        let dir = Vec3::new(0., 1., 0.);
        *pos = Vec3::new(0., 0.5, 0.) + dir * (time.seconds_since_startup().sin() as f32);
    }
}