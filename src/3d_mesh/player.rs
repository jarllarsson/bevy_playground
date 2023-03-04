use bevy::{prelude::*};
use bevy_prototype_debug_lines::*;
use std::path::Path;
use const_format::concatcp;
use crate::{Player, animation::AnimationLink, Camera, CameraRotation, Speed, MyCustomMaterial, /*SystemOrder,*/ 
/*GAMEPAD_DEADZONE, GAMEPAD_AXIS_L_SENSITIVITY*/};

const PLAYER_MESH_PATH: &str = "models/Fox.glb";

pub struct PlayerPlugin;

#[derive(Resource)]
struct PlayerAnimations
{
    walk: Handle<AnimationClip>,
    idle: Handle<AnimationClip>,
    run:  Handle<AnimationClip>,
}

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_stage(
            "setup_player",
            SystemStage::single(player_spawn))
        /*.add_system(player_movement.label(SystemOrder::PlayerMovement))*/
        .add_system(player_animation);
    }
}

fn player_spawn(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<MyCustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Make a player sphere
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(concatcp!(PLAYER_MESH_PATH, "#Scene0")),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(0.01, 0.01, 0.01),
                ..default()
            },
            ..default()
        },
        Name::new("Player")
    ))
    // Custom components
    .insert(Speed::default())
    .insert(Player);

    commands.insert_resource(PlayerAnimations {
        walk: asset_server.load(concatcp!(PLAYER_MESH_PATH, "#Animation2")),
        idle: asset_server.load(concatcp!(PLAYER_MESH_PATH, "#Animation1")),
        run:  asset_server.load(concatcp!(PLAYER_MESH_PATH, "#Animation0")),
    });
}

/*
fn player_movement(
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
    mut lines: ResMut<DebugLines>,
    mut player_query: Query<(&mut Speed, &mut Transform), (With<Player>, Without<Camera>)>,
    camera_query: Query<&CameraRotation, (With<Camera>, Without<Player>)>
){  
    if let Ok((mut speed, mut transform)) = player_query.get_single_mut() {
        let dt = time.delta_seconds();
        // Construct input vector from keyboard presses
        let mut move_input = Vec3::new(
            if kb_input.pressed(KeyCode::A) {-1.} else if kb_input.pressed(KeyCode::D) {1.} else {0.},  // Sideways (X is right)
            0.,
            if kb_input.pressed(KeyCode::W) {-1.} else if kb_input.pressed(KeyCode::S) {1.} else {0.}); // Forward/Backward (-Z is forward)
        // if we have a gamepad, let it override input
        for gamepad in gamepads.iter() {
            let move_input_raw = Vec3::new(
                gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap(),
                0.,
                gamepad_axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)).unwrap());
            if move_input_raw.length_squared() > GAMEPAD_DEADZONE * GAMEPAD_DEADZONE {
                move_input.x = move_input_raw.x.abs().powf(GAMEPAD_AXIS_L_SENSITIVITY) * move_input_raw.x.signum();
                move_input.z = move_input_raw.z.abs().powf(GAMEPAD_AXIS_L_SENSITIVITY) * move_input_raw.z.signum() * -1.;
            }
        }

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
*/

fn player_animation(
    animations: Res<PlayerAnimations>,
    mut anim_players_query: Query<&mut AnimationPlayer>,
    query: Query<(&Speed, &AnimationLink), With<Player>>,
){
    for (speed, anim_link) in query.iter() {
        if let Ok(mut anim) = anim_players_query.get_mut(anim_link.0) {
            anim.play(animations.idle.clone_weak()).repeat();
        }
    }
}