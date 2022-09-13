// Very simple first test of a compute shader that calculates a colour
// Based on compute shader examples for bevy

use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        RenderApp, RenderStage,
    },
};
// Moo. "clone on write", ie keep a ref until change is needed, then clone (https://doc.rust-lang.org/std/borrow/enum.Cow.html)
use std::borrow::Cow;

// Compute shader dimensions

// Total threads X*Y
const SIZE: (u32, u32) = (640, 480);
// Threads per group X*X
const WORKGROUP_SIZE: u32 = 8;

// Types

// Custom struct for tracking the render target
// Derives clone so its internals are deep copied,
// Deref to get the Image from handle (struct must be single-item for this!)
// and ExtractResource in order to be able to extract the image from bevy's main/game "world" to its render "world"
#[derive(Clone, Deref, ExtractResource)]
struct MyComputeShaderRenderTarget(Handle<Image>);

// Custom struct containing bind group of resources for our shader.
struct  MyComputeShaderRenderTargetBindGroup(BindGroup);


// Setup boilerplate
// Program entry point and resource setup

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK)) // Our global clear color
    .add_plugins(DefaultPlugins)
    .add_plugin(MyComputeShaderPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands, 
    mut images: ResMut<Assets<Image>>
) {
    // Create main presentation texture and compute render target resource...
    let mut image = Image::new_fill(
        Extent3d { width: SIZE.0, height: SIZE.1, depth_or_array_layers: 1, },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage = 
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    // ...and add it to our image asset server
    let image = images.add(image);
    
    // Setup the image to be rendered as a sprite to screen
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    // Add image handle as a resource (of our type) to track
    commands.insert_resource(MyComputeShaderRenderTarget(image));

    // 2d camera for just displaying the texture
    commands.spawn_bundle(Camera2dBundle::default());
}

// Compute shader plugin
// Here is where we encapsulate all our compute shader stuff

pub struct MyComputeShaderPlugin;

impl Plugin for MyComputeShaderPlugin {
    // Plugin setup on app startup
    fn build(&self, app: &mut App) {
        // Extract the render target on which the compute shader needs access to.
        // From main world to render world.
        app.add_plugin(ExtractResourcePlugin::<MyComputeShaderRenderTarget>::default());

        // Create our custom render pipeline and a bind group stage
        // Pipeline describes stages (shaders) of a custom graphics pipeline.
        // Bind groups binds resources to the shaders.
        let render_app = app.sub_app_mut(RenderApp); // fetch sub app "RenderApp"
        render_app
            .init_resource::<MyComputeShaderPipeline>()
            .add_system_to_stage(RenderStage::Queue, queue_bind_group);

        // Create render graph node for our shader.
        // It defines the dependencies our shader and its resources has to others, and schedules it.
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        const my_compute_node_name: &str = "my_compute";
        // Make the node
        render_graph.add_node(my_compute_node_name, MyComputeNode::default());
        // Schedule node to run before the camera node, check for OK with unwrap (panics if not)
        render_graph.add_node_edge(my_compute_node_name, bevy::render::main_graph::node::CAMERA_DRIVER).unwrap();
    }
}

// Our bind group enqueueing function/system that is added to the Bevy "Queue" render stage in the plugin setup.
// Queues the bind group that exist in the pipeline
fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<MyComputeShaderPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    render_target: Res<MyComputeShaderRenderTarget>,
    device: Res<RenderDevice>,
) {
    // Fetch gpu view of our render target.
    // We can use * on render_target to get the handle to borrow as MyComputeShaderRenderTarget derives Deref (otherwise use .0).
    let view = &gpu_images[&*render_target];
    // Bind the view to a new bind group (I assume if we have more resources we add them to the same group as make sense based on lifetimes)
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("RenderTextureBindGroup"),
        layout: &pipeline.texture_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&view.texture_view),
        }],
    });
    commands.insert_resource(MyComputeShaderRenderTargetBindGroup(bind_group))
}

// Custom struct defining the pipeline, contains references to the bind groups that binds the resources needed
// and the pipelines for initializing and updating.
pub struct MyComputeShaderPipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

// implement the FromWorld trait on our pipeline, which allows it to
// initialize from a given world context when created as a resource to the RenderApp
impl FromWorld for MyComputeShaderPipeline {
    // Override the from_world function to do setups when given world context
    fn from_world(world: &mut World) -> Self {
        // Setup members of struct
        // Define the layout of the bind group, ie. the members to bind to the shader.
        // This layout is referenced when queuing the bind group to the shader.
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("RenderTextureBindGroup_Layout"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    }],
                });
        let shader = 
    }
}