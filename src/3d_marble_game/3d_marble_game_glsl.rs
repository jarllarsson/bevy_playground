// A simple 3d marble game
// Goals:
// * A sphere controllable by the player with input. DONE
// * A camera that follows the sphere.
// * The sphere has physics - Use Rapier for physics?
// * The sphere falls off the world when outside and game restarts.

// Project module declaration (same as file names)
mod player;
mod camera;

use lerp::num_traits::clamp;
// Includes from project modules
use player::PlayerPlugin;
use camera::CameraPlugin;

// External includes

use std::{
    f32::consts::PI,
};

use rand::{
    thread_rng, 
    Rng
};

// Bevy includes

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    }, math::Vec3Swizzles,
};

use bevy_prototype_debug_lines::*;

// Component types

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct FloorTile;

#[derive(Component)]
struct Speed(Vec3);
impl Default for Speed
{
    fn default() -> Self {
        Self(Vec3::splat(0.))
    }
}

#[derive(Component)]
struct CameraRotation(Vec2);
impl Default for CameraRotation
{
    fn default() -> Self {
        Self(Vec2::splat(0.))
    }
}

// Update order labels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum SystemType {
    PlayerMovement,
    CameraMovement,
}

// Global constants
const MARBLE_RADIUS: f32 = 1.;
const FLOOR_TILE_NUM: u8 = 10;
const FLOOR_TILE_SIZE: f32 = 8.;
const FLOOR_SIZE: f32 = FLOOR_TILE_NUM as f32 * FLOOR_TILE_SIZE;
const FLOOR_POSITION: Vec3 = Vec3::new(-FLOOR_SIZE * 0.5, -FLOOR_TILE_SIZE * 0.5, -FLOOR_SIZE * 0.5);

// App entry point

fn main() {
    // Setup and run Bevy
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<MyCustomMaterial>::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_startup_system(setup)
        .add_system(floor_magic)
        // .add_system(cube_animation)
        .run();
}

// Main setup

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Light the sphere
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: 100000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    for x in 0..FLOOR_TILE_NUM-1 {
        for z in 0..FLOOR_TILE_NUM-1 {
            let x_norm = x as f32 / FLOOR_TILE_NUM as f32;
            let z_norm = z as f32 / FLOOR_TILE_NUM as f32;
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: FLOOR_TILE_SIZE })),
                material: materials.add( StandardMaterial {
                    base_color:         Color::rgb(x_norm, 0.0, z_norm),
                    base_color_texture: Some(asset_server.load("cobblestone.png")),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(x as f32 * FLOOR_TILE_SIZE, 0., z as f32 * FLOOR_TILE_SIZE) + FLOOR_POSITION),
                ..default()
            })
            .insert(FloorTile);
        }
    }

    /* Log examples
    error!("Unknown condition!");
    warn!("Something unusual happened!");
    info!("Entered game level: {}", 2);
    debug!("x: {}, state: {:?}", 0.1, "test");
    trace!("entity transform: {:?}", Transform::from_xyz(-2.0, 2.5, 5.0));*/
}

fn floor_magic(
    time: Res<Time>,
    mut floor_query: Query<&mut Transform, (With<FloorTile>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<FloorTile>)>,
){
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation;
        for mut cube_transform in floor_query.iter_mut() {
            let time_sine = time.elapsed_seconds().sin() as f32;

            let dist_to_player = player_translation.xz().distance(cube_transform.translation.xz());
            let dist_to_player_floorspace = ((dist_to_player / FLOOR_TILE_SIZE).floor() - 2.).max(0.);
            // cube_transform.translation.y = FLOOR_POSITION.y + time_sine * dist_to_player_floorspace;
            // cube_transform.rotation = Quat::from_rotation_y((time_sine + 1.0) * PI);
        }
    }
}

/*fn cube_animation(
    time: Res<Time>,
    mut materials: ResMut<Assets<MyCustomMaterial>>,
    mut query: Query<(&Handle<MyCustomMaterial>, &mut Transform, With<Player>)>
){
    for (mat_handle, mut cube_transform, _) in query.iter_mut() {
        let time_sine = time.elapsed_seconds().sin() as f32;
        let dir = Vec3::new(0., 1., 0.);
        cube_transform.translation = Vec3::new(0., 0.5, 0.) + dir * time_sine;
        cube_transform.rotation = Quat::from_rotation_y((time_sine + 1.0) * PI);
        if let Some(mat) = materials.get_mut(mat_handle) {
            let mut rng = thread_rng();
            mat.time = time_sine;
            if rng.gen_bool(1.0 / 30.0) {
                mat.color = Color::rgba(
                    rng.gen_range(50..100) as f32 * 0.01, 
                    rng.gen_range(50..100) as f32 * 0.01, 
                    rng.gen_range(50..100) as f32 * 0.01, 
                    1.0
                );
            }
        }
    }
}*/

// Shader buffer bindings
// https://docs.rs/bevy/0.8.0/bevy/render/render_resource/trait.AsBindGroup.html
#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "f7bd480f-cf1c-4d67-bf96-98bcedc996c0"]
pub struct MyCustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    time: f32,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

// Implement the material trait for our custom material struct in order to make it compliant with shader pipeline.
// Override the behaviours for which we don't want the default behaviours.
 impl Material for MyCustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.frag".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // Specify shader program entrypoint overrides (not needed for WGSL)
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> 
    {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
 }