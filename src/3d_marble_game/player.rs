use bevy::{prelude::*, math::Vec3Swizzles};
use lerp::Lerp;
use crate::{Player};


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
    mut commands: Commands
) {
    // Create entity and add to entity bundle
    /*
    commands.spawn(SpriteBundle {
        texture: sprite_images.player.0.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 400., 10.),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    // Custom components
    .insert(Player); // Unit struct to define player type
    .insert(Speed { 0: Vec2::new(0., 0.) })
    .insert(OldPos::default())
    .insert(PlayerReadyFire(true));*/
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(/*&mut Speed, */&mut Transform, With<Player>)>
){
    /*
    if let Ok((mut speed, mut transform, _)) = query.get_single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.0
        };

        let old_pos = transform.translation;
        speed.0.x += 500.0 * dir * TIME_STEP;
        speed.0.x =  speed.0.x.lerp(0., 1. - 0.5f32.powf(TIME_STEP));
        if speed.0.x > 100. { speed.0.x = 100. };
        if speed.0.x < -100. { speed.0.x = -100.};

        transform.translation.x += speed.0.x * TIME_STEP;
        speed.0.y -= 1000. * TIME_STEP;
        transform.translation.y += speed.0.y * TIME_STEP;

        let pos = &mut transform.translation;
        let area = Vec3::new(win_size.w / 2., win_size.h / 2., 0.);
        
        if pos.x > area.x || pos.x < -area.x
        {
            pos.x = old_pos.x;
        }
        if pos.y > area.y || pos.y < -area.y
        {
            pos.y = old_pos.y;
        }
    }
    */
}
