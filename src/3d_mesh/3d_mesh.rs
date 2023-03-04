// Render a 3d mesh
// Goals:
// 1 First get a static mesh working
// 2 Get skinning and animation working
// 3 Animation states
// 4 Animation states with blending


// Project module declaration (same as file names)
mod player;
mod camera;
mod animation;

// Includes from project modules
use player::PlayerPlugin;
use camera::CameraPlugin;
use animation::AnimationPlugin;

// External includes

use std::{
    f32::consts::PI,
};

// Bevy includes

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypeUuid,        // For material id
        Reflect, TypeRegistry},// For reflecting data to egui
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    }
};
use bevy_prototype_debug_lines::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Component types

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct FloorTile;

#[derive(Reflect, Component)]
#[reflect(Component)]
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
enum SystemOrder {
    PlayerMovement,
    CameraMovement,
}



// Global constants
const MARBLE_RADIUS: f32 = 1.;
const FLOOR_SIZE: Vec3 = Vec3::new(80., 8., 80.);
const FLOOR_POSITION: Vec3 = Vec3::new(0., -FLOOR_SIZE.y * 0.5, 0.);
const GAMEPAD_DEADZONE: f32 = 0.1;
const GAMEPAD_AXIS_L_SENSITIVITY: f32 = 1.5;
const GAMEPAD_AXIS_R_SENSITIVITY: f32 = 5.5;

// App entry point

fn main() {
    // Setup and run Bevy
    App::new()
        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            }))
        .add_plugin(MaterialPlugin::<MyCustomMaterial>::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup)
        // EGUI Type registry
        .register_type::<Speed>()
        // Let's go
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


    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(FLOOR_SIZE.x, FLOOR_SIZE.y, FLOOR_SIZE.z))),
            material: materials.add( StandardMaterial {
                base_color:         Color::SEA_GREEN,
                base_color_texture: Some(asset_server.load("cobblestone.png")),
                ..default()
            }),
            transform: Transform::from_translation(FLOOR_POSITION),
            ..default()
        },
        Name::new("Floor")
    ))
    .insert(FloorTile);

    /* Log examples
    error!("Unknown condition!");
    warn!("Something unusual happened!");
    info!("Entered game level: {}", 2);
    debug!("x: {}, state: {:?}", 0.1, "test");
    trace!("entity transform: {:?}", Transform::from_xyz(-2.0, 2.5, 5.0));*/
}



// Shader buffer bindings
// https://docs.rs/bevy/0.8.0/bevy/render/render_resource/trait.AsBindGroup.html
#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "69196246-07cd-4581-9885-167958593672"]
pub struct MyCustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    time: f32,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    noise_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

// Implement the material trait for our custom material struct in order to make it compliant with shader pipeline.
// Override the behaviours for which we don't want the default behaviours.
 impl Material for MyCustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_marble.frag".into()
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