// A simple 3d marble game
// Goals:
// * A sphere controllable by the player with input. DONE
// * A camera that follows the sphere.
// * The sphere has physics - Use Rapier for physics?
// * The sphere falls off the world when outside and game restarts.

// Project module declaration (same as file names)
mod player;
mod camera;

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
    },
};

use bevy_prototype_debug_lines::*;

// Component types

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Speed(Vec3);
impl Default for Speed
{
    fn default() -> Self {
        Self(Vec3::new(0., 0., 0.))
    }
}

#[derive(Component)]
struct Angle(f32);

// Update order labels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum SystemType {
    PlayerMovement,
    CameraMovement,
}

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
        // .add_system(cube_animation)
        .run();
}

// Main setup

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    let square_num = 10;
    let square_size = 2.;
    let offset = square_num as f32 * square_size * 0.5;
    for x in 0..square_num-1 {
        for z in 0..square_num-1 {
            let x_norm = x as f32 / square_num as f32;
            let z_norm = z as f32 / square_num as f32;
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: square_size })),
                material: materials.add(Color::rgb(x_norm, 0.0, z_norm).into()),
                transform: Transform::from_xyz(x as f32 * square_size - offset, -1., z as f32 * square_size - offset),
                ..default()
            });
        }
    }

    /* Log examples
    error!("Unknown condition!");
    warn!("Something unusual happened!");
    info!("Entered game level: {}", 2);
    debug!("x: {}, state: {:?}", 0.1, "test");
    trace!("entity transform: {:?}", Transform::from_xyz(-2.0, 2.5, 5.0));*/
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