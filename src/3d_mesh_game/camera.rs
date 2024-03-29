use bevy::{prelude::*};
use bevy_prototype_debug_lines::*;
use crate::{Player, Camera, CameraRotation, Speed, SystemOrder, 
    GAMEPAD_DEADZONE, GAMEPAD_AXIS_R_SENSITIVITY};
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
    gamepads: Res<Gamepads>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
    mut lines: ResMut<DebugLines>,
    mut camera_query: Query<(&mut Transform, &mut CameraRotation, &mut Speed), (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>
){  
    if let Ok((mut camera_transform, mut camera_angle, mut speed)) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {

            let dt = time.delta_seconds();
            // Construct input vector from keyboard presses
            let mut move_input = Vec3::new(
                if kb_input.pressed(KeyCode::Left) {-1.} else if kb_input.pressed(KeyCode::Right) {1.} else {0.0},
                if kb_input.pressed(KeyCode::Up) {1.} else if kb_input.pressed(KeyCode::Down)   {-1.} else {0.0},
                0.);

            // if we have a gamepad, let it override input
            for gamepad in gamepads.iter() {
                let move_input_raw = Vec3::new(
                    gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)).unwrap(),
                    gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY)).unwrap(),
                    0.);
                if move_input_raw.length_squared() > GAMEPAD_DEADZONE * GAMEPAD_DEADZONE {
                    move_input.x = move_input_raw.x.abs().powf(GAMEPAD_AXIS_R_SENSITIVITY) * move_input_raw.x.signum();
                    move_input.y = move_input_raw.y.abs().powf(GAMEPAD_AXIS_R_SENSITIVITY) * move_input_raw.y.signum();
                }
            }
            
            let max_angle = 2. * PI;

            let update_angle_wrapped = |current_angle : f32, delta_angle : f32| -> f32 {
                let mut new_angle = current_angle + delta_angle;
                if new_angle > max_angle {new_angle -= max_angle;}
                if new_angle < 0.        {new_angle += max_angle;}
                new_angle
            };
            camera_angle.0.y = update_angle_wrapped(camera_angle.0.y, -move_input.x * dt);
            camera_angle.0.x = (camera_angle.0.x + move_input.y * dt).clamp(-1.0, 0.1);
                
            // Place behind player and look to center
            let center = player_transform.translation.clone();
            let offset = Quat::from_rotation_y(camera_angle.0.y) * Quat::from_rotation_x(camera_angle.0.x) * Vec3::new(0., 0., 5.0);
            *camera_transform = Transform::from_translation(center + offset).looking_at(center, Vec3::Y);

            
            let line_start_2d = Vec3::new(0., 0., -1.0);
            let line_end_2d = line_start_2d + move_input.normalize() * 0.2;
            lines.line_gradient(camera_transform.transform_point(line_start_2d), camera_transform.transform_point(line_end_2d), 0., 
            Color::RED, Color::LIME_GREEN);
        }
    }
}
