use bevy::{prelude::*};
use bevy_prototype_debug_lines::*;
use lerp::Lerp;
use crate::{Player, Camera, CameraRotation, Speed, SystemOrder};
use std::{f32::consts::PI};
pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "setup_camera",
            SystemStage::single(camera_spawn))
        .add_system(
            camera_movement
            .after(SystemOrder::PlayerMovement)
            .label(SystemOrder::CameraMovement)
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
    .insert(CameraRotation::default())
    .insert(Speed::default())
    .insert(Camera);
}

fn camera_movement(
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    mut lines: ResMut<DebugLines>,
    mut camera_query: Query<(&mut Transform, &mut CameraRotation, &mut Speed), (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>
){  
    if let Ok((mut camera_transform, mut camera_angle, mut speed)) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {

            let dt = time.delta_seconds();
            // Construct input vector from keyboard presses
            let move_input = Vec3::new(
                if kb_input.pressed(KeyCode::Left) {1.} else if kb_input.pressed(KeyCode::Right) {-1.} else {0.0},
                if kb_input.pressed(KeyCode::Down) {-1.} else if kb_input.pressed(KeyCode::Up)    {1.} else {0.0},
                0.);
            
            let max_angle = 2. * PI;
            // Accelerate
            speed.0 += 20.0 * move_input * dt;
            // Friction
            let friction_t = 1. - 0.01f32.powf(dt);
            speed.0 = speed.0.lerp(Vec3::splat(0.), friction_t);
            // Clamp max speed
            let max_speed = 2.;
            speed.0 = speed.0.clamp(Vec3::splat(-max_speed), Vec3::splat(max_speed));

            let update_angle_wrapped = |current_angle : f32, delta_angle : f32| -> f32 {
                let mut new_angle = current_angle + delta_angle;
                if new_angle > max_angle {new_angle -= max_angle;}
                if new_angle < 0.        {new_angle += max_angle;}
                new_angle
            };
            camera_angle.0.y = update_angle_wrapped(camera_angle.0.y, speed.0.x * dt);
            camera_angle.0.x = (camera_angle.0.x + speed.0.y * dt).clamp(-1.0, 0.2);
                
            // Place behind player and look to center
            let center = player_transform.translation.clone();
            let offset = Quat::from_rotation_y(camera_angle.0.y) * Quat::from_rotation_x(camera_angle.0.x) * Vec3::new(0., 0., 5.0);
            *camera_transform = Transform::from_translation(center + offset).looking_at(center, Vec3::Y);
        }
    }
}
