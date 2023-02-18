use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_prototype_debug_lines::*;
use lerp::Lerp;
use crate::{Player, Speed, MyCustomMaterial};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn))
        .add_system(player_movement);
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
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, sectors: 10, stacks: 10 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(MyCustomMaterial {
            color: Color::BLUE,
            time: 0.0,
            color_texture: Some(asset_server.load("block.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    })
    // Custom components
    .insert(Speed { 0: Vec3::splat(0.) })
    .insert(Player);
}

fn player_movement(
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    mut lines: ResMut<DebugLines>,
    mut query: Query<(&mut Speed, &mut Transform, With<Player>)>
){  
    if let Ok((mut speed, mut transform, _)) = query.get_single_mut() {
        let dt = time.delta_seconds();
        // Construct input vector from keyboard presses
        let move_input = Vec2::new(
            if kb_input.pressed(KeyCode::Left) {-1.} else if kb_input.pressed(KeyCode::Right) {1.} else {0.0},
            if kb_input.pressed(KeyCode::Down) {-1.} else if kb_input.pressed(KeyCode::Up) {1.} else {0.0});

        // Accelerate
        speed.0.x += 1.0 * move_input.x * dt; // Sideways (X is right)
        speed.0.z -= 1.0 * move_input.y * dt; // Forward/Backward (-Z is forward)
         // Friction
        let friction_t = 1. - 0.5f32.powf(dt);
        speed.0 =  speed.0.lerp(Vec3::splat(0.), friction_t);
        // Clamp max speed
        let max_speed = 100.;
        if speed.0.x > max_speed { speed.0.x = max_speed } else if speed.0.x < -max_speed { speed.0.x = -max_speed};
        if speed.0.z > max_speed { speed.0.z = max_speed } else if speed.0.z < -max_speed { speed.0.z = -max_speed};

        // Update position with velocity (arc=a*r)
        transform.translation += speed.0 * dt;
        let radius = 1.;
        let rot_speed_z = -(speed.0.x).clamp(-1., 1.).asin();
        let rot_speed_x = (speed.0.z).clamp(-1., 1.).asin();
        transform.rotation = Quat::from_rotation_z( radius * rot_speed_z * dt) * transform.rotation;
        transform.rotation = Quat::from_rotation_x( radius * rot_speed_x * dt) * transform.rotation;
        info!("Rotation: {:?}", transform.rotation);
        lines.line_gradient(transform.translation, transform.translation + transform.up().normalize() * 2., 0., 
            Color::AZURE, Color::FUCHSIA);
    }
    
}
