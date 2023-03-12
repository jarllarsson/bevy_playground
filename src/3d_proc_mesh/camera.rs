use bevy::{prelude::*};
use bevy_prototype_debug_lines::*;
use crate::{Camera, CameraRotation, Speed, 
    GAMEPAD_DEADZONE, GAMEPAD_AXIS_R_SENSITIVITY, GAMEPAD_AXIS_L_SENSITIVITY};
use std::{f32::consts::PI};
pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "setup_camera",
            SystemStage::single(camera_spawn))
        .add_system(camera_movement);
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
    gamepad_buttons: Res<Input<GamepadButton>>,
    mut lines: ResMut<DebugLines>,
    mut camera_query: Query<(&mut Transform, &mut CameraRotation, &mut Speed), With<Camera>>
) {  
    if let Ok((mut camera_transform, mut camera_angle, mut speed)) = camera_query.get_single_mut() {
        let dt = time.delta_seconds();
        // Construct input vector from keyboard presses
        let mut rotate_input = Vec3::new(
            if kb_input.pressed(KeyCode::Left) {-1.} else if kb_input.pressed(KeyCode::Right) {1.} else {0.0},
            if kb_input.pressed(KeyCode::Up) {1.} else if kb_input.pressed(KeyCode::Down)   {-1.} else {0.0},
            0.);
        let mut fly_input = Vec3::new(
            if kb_input.pressed(KeyCode::A) {-1.} else if kb_input.pressed(KeyCode::D) {1.} else {0.0},
            if kb_input.pressed(KeyCode::R) {1.} else if kb_input.pressed(KeyCode::F)   {-1.} else {0.0},
            if kb_input.pressed(KeyCode::W) {-1.} else if kb_input.pressed(KeyCode::S)   {1.} else {0.0});

        // if we have a gamepad, let it override input
        for gamepad in gamepads.iter() {
            let rotate_input_raw = Vec3::new(
                gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)).unwrap(),
                gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY)).unwrap(),
                0.);
            let fly_input_raw = Vec3::new(
                gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap(),
                if gamepad_buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {1.} 
                else if gamepad_buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {-1.} else {0.0},
                gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)).unwrap());
            if rotate_input_raw.length_squared() > GAMEPAD_DEADZONE * GAMEPAD_DEADZONE {
                rotate_input.x = rotate_input_raw.x.abs().powf(GAMEPAD_AXIS_R_SENSITIVITY) * rotate_input_raw.x.signum();
                rotate_input.y = rotate_input_raw.y.abs().powf(GAMEPAD_AXIS_R_SENSITIVITY) * rotate_input_raw.y.signum();
            }
            if Vec3::new(fly_input_raw.x, fly_input_raw.z, 0.).length_squared() > GAMEPAD_DEADZONE * GAMEPAD_DEADZONE {
                fly_input.x = fly_input_raw.x.abs().powf(GAMEPAD_AXIS_L_SENSITIVITY) * fly_input_raw.x.signum();
                fly_input.z = fly_input_raw.z.abs().powf(GAMEPAD_AXIS_L_SENSITIVITY) * fly_input_raw.z.signum() * -1.;
            }
            fly_input.y = fly_input_raw.y;
        }
        
        let max_angle = 2. * PI;

        let update_angle_wrapped = |current_angle : f32, delta_angle : f32| -> f32 {
            let mut new_angle = current_angle + delta_angle;
            if new_angle > max_angle {new_angle -= max_angle;}
            if new_angle < 0.        {new_angle += max_angle;}
            new_angle
        };
        let rot_speed = 1.5;
        camera_angle.0.y = update_angle_wrapped(camera_angle.0.y, -rotate_input.x * rot_speed * dt);
        camera_angle.0.x = (camera_angle.0.x + rotate_input.y * rot_speed * dt).clamp(-1.0, 1.);

        // Transform input to world space
        let rotation = Quat::from_rotation_y(camera_angle.0.y) * Quat::from_rotation_x(camera_angle.0.x);
        let world_fly_plane_input = rotation * Vec3::new(fly_input.x, 0., fly_input.z);
        fly_input = Vec3::new(world_fly_plane_input.x, world_fly_plane_input.y + fly_input.y, world_fly_plane_input.z);

        // Accelerate
        speed.0 += 50.0 * fly_input * dt;
         // Friction
        let friction_t = 1. - 0.05f32.powf(dt);
        speed.0 = speed.0.lerp(Vec3::splat(0.), friction_t);
        // Clamp max speed
        let max_speed = 30.;
        let speed_magnitude = speed.0.length().min(max_speed);
        let speed_dir = speed.0.normalize_or_zero();            // Can still have magnitude when move_dir = 0
        speed.0 = speed_dir * speed_magnitude;

        // Update position with velocity (arc=a*r)
        camera_transform.translation += speed.0 * dt;
        camera_transform.rotation = rotation;

        
        let line_start_2d = Vec3::new(0., 0., -1.0);
        let line_end_2d = line_start_2d + rotate_input.normalize() * 0.2;
        lines.line_gradient(camera_transform.transform_point(line_start_2d), camera_transform.transform_point(line_end_2d), 0., 
        Color::RED, Color::LIME_GREEN);
    }
}
