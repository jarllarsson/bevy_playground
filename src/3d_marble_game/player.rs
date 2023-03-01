use bevy::{prelude::*};
use bevy_prototype_debug_lines::*;
use crate::{Player, Camera, CameraRotation, Speed, MyCustomMaterial, SystemOrder, MARBLE_RADIUS};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "setup_player",
            SystemStage::single(player_spawn))
        .add_system(player_movement.label(SystemOrder::PlayerMovement))
        .add_system(player_animation);
    }
}

fn player_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyCustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Make a player sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: MARBLE_RADIUS, sectors: 20, stacks: 20 })),
        transform: Transform::from_xyz(0.0, MARBLE_RADIUS, 0.0),
        material: materials.add(MyCustomMaterial {
            color: Color::BLUE,
            time: 0.0,
            color_texture: Some(asset_server.load("ball.png")),
            noise_texture: Some(asset_server.load("manifold_noise.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    })
    // Custom components
    .insert(Speed::default())
    .insert(Player);
}

fn player_movement(
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    mut lines: ResMut<DebugLines>,
    mut player_query: Query<(&mut Speed, &mut Transform), (With<Player>, Without<Camera>)>,
    camera_query: Query<(&CameraRotation), (With<Camera>, Without<Player>)>
){  
    if let Ok((mut speed, mut transform)) = player_query.get_single_mut() {
        let dt = time.delta_seconds();
        // Construct input vector from keyboard presses
        let mut move_input = Vec3::new(
            if kb_input.pressed(KeyCode::A) {-1.} else if kb_input.pressed(KeyCode::D) {1.} else {0.},  // Sideways (X is right)
            0.,
            if kb_input.pressed(KeyCode::W) {-1.} else if kb_input.pressed(KeyCode::S) {1.} else {0.}); // Forward/Backward (-Z is forward)

        // Transform input to world space
        if let Ok(angle) = camera_query.get_single() {
            move_input = Quat::from_rotation_y(angle.0.y) * move_input;
        }

        // Accelerate
        speed.0 += 8.0 * move_input * dt;
         // Friction
        let friction_t = 1. - 0.5f32.powf(dt);
        speed.0 =  speed.0.lerp(Vec3::splat(0.), friction_t);
        // Clamp max speed
        let max_speed = 100.;
        speed.0 = speed.0.clamp(Vec3::splat(-max_speed), Vec3::splat(max_speed));

        // Update position with velocity (arc=a*r)
        transform.translation += speed.0 * dt;
        let radius = 1.;
        let rot_speed_z = -(speed.0.x / max_speed).asin() * max_speed;
        let rot_speed_x = (speed.0.z / max_speed).asin() * max_speed;
        transform.rotation = Quat::from_rotation_z( radius * rot_speed_z * dt) * transform.rotation;
        transform.rotation = Quat::from_rotation_x( radius * rot_speed_x * dt) * transform.rotation;

        lines.line_gradient(transform.translation, transform.translation + transform.up().normalize() * 2., 0., 
            Color::AZURE, Color::FUCHSIA);
        lines.line_gradient(transform.translation, transform.translation + move_input * 2., 0., 
            Color::GREEN, Color::ORANGE);
    }
    
}

fn player_animation(
    time: Res<Time>,
    mut materials: ResMut<Assets<MyCustomMaterial>>,
    mut query: Query<&Handle<MyCustomMaterial>, With<Player>>
){
    for (mat_handle) in query.iter_mut() {
        let time_sine = time.elapsed_seconds() as f32;
        if let Some(mat) = materials.get_mut(mat_handle) {
            mat.time = time_sine;
        }
    }
}