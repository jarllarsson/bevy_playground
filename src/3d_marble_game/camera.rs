use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_prototype_debug_lines::*;
use lerp::Lerp;
use crate::{Player, Camera, Angle, SystemType};
use std::{f32::consts::PI};
pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "setup_camera",
            SystemStage::single(camera_spawn))
        .add_system(
            camera_movement
            .after(SystemType::PlayerMovement)
            .label(SystemType::CameraMovement)
        );
    }
}

fn camera_spawn(
    mut commands: Commands,
) {
    // Make a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    // Custom components
    .insert(Angle{ 0: 0. })
    .insert(Camera);
}

fn camera_movement(
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    mut lines: ResMut<DebugLines>,
    mut camera_query: Query<(&mut Transform, &mut Angle), (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>
){  
    if let Ok((mut camera_transform, mut camera_angle)) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {

            let dt = time.delta_seconds();
            // Construct input vector from keyboard presses
            let move_input = Vec2::new(
                if kb_input.pressed(KeyCode::Left) {1.} else if kb_input.pressed(KeyCode::Right) {-1.} else {0.0},
                if kb_input.pressed(KeyCode::Down) {-1.} else if kb_input.pressed(KeyCode::Up) {1.} else {0.0});
            
            camera_angle.0 += move_input.x * 3. * dt;
            let max_angle = 2. * PI;
            if camera_angle.0 > max_angle {camera_angle.0 -= max_angle;}
            if camera_angle.0 < 0.        {camera_angle.0 += max_angle;}
                
            // Place behind player and look to center
            let center = player_transform.translation.clone();
            let offset = Quat::from_rotation_y(camera_angle.0) * Vec3::new(0., 2.5, 5.0);
            *camera_transform = Transform::from_translation(center + offset).looking_at(center, Vec3::Y);
            /*
            // Update position with velocity (arc=a*r)
            transform.translation += speed.0 * dt;
            let radius = 1.;
            let rot_speed_z = -(speed.0.x).clamp(-1., 1.).asin();
            let rot_speed_x = (speed.0.z).clamp(-1., 1.).asin();
            transform.rotation = Quat::from_rotation_z( radius * rot_speed_z * dt) * transform.rotation;
            transform.rotation = Quat::from_rotation_x( radius * rot_speed_x * dt) * transform.rotation;
            info!("Rotation: {:?}", transform.rotation);
            lines.line_gradient(transform.translation, transform.translation + transform.up().normalize() * 2., 0., 
                Color::AZURE, Color::FUCHSIA);*/
        }
    }
}
