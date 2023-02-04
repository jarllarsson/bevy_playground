use bevy::{prelude::*, math::Vec3Swizzles};
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
    .insert(Speed { 0: Vec2::new(0., 0.) })
    .insert(Player);
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Speed, &mut Transform, With<Player>)>
){  
    if let Ok((mut speed, mut transform, _)) = query.get_single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.0
        };
        /*let xdir = if keyboard_input.pressed(KeyCode::Up) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Down) {
            1.
        } else {
            0.0
        };*/
        let dt = time.delta_seconds();
        let old_pos = transform.translation;

        // Accelerate
        speed.0.x += 1.0 * dir * dt;
        speed.0.x =  speed.0.x.lerp(0., 1. - 0.5f32.powf(dt)); // Friction
        if speed.0.x > 100. { speed.0.x = 100. };
        if speed.0.x < -100. { speed.0.x = -100.};

        // Update position with velocity
        transform.translation.x += speed.0.x * dt;
        // speed.0.y -= 1000. * dt;
        // transform.translation.y += speed.0.y * dt;

        let pos = &mut transform.translation;
        /*let area = Vec3::new(win_size.w / 2., win_size.h / 2., 0.);
        
        if pos.x > area.x || pos.x < -area.x
        {
            pos.x = old_pos.x;
        }
        if pos.y > area.y || pos.y < -area.y
        {
            pos.y = old_pos.y;
        }*/
    }
    
}
