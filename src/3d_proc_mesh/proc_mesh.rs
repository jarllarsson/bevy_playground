use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology
    }
};
use crate::{ProcMesh, MyCustomMaterial};
use bevy_prototype_debug_lines::*;
use fast_surface_nets::glam::{Vec2, Vec3A};
use fast_surface_nets::ndshape::{ConstShape, ConstShape3u32};
use fast_surface_nets::{surface_nets, SurfaceNetsBuffer};

pub struct ProcMeshPlugin;

impl Plugin for ProcMeshPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_stage(
            "generate_mesh",
            SystemStage::single(gen_mesh))
        .add_system(update_mesh);
    }
}

// A 32^3 chunk with 1-voxel boundary padding.
type SampleShape = ConstShape3u32<34, 34, 34>;

fn generate_mesh(meshes: &mut Assets<Mesh>,
    sdf: impl Fn(Vec3A) -> f32,
) -> (SurfaceNetsBuffer, Handle<Mesh>) {

    let mut samples = [1.0; SampleShape::SIZE as usize];
    for i in 0u32..(SampleShape::SIZE) {
        let p = into_domain(32, SampleShape::delinearize(i));
        samples[i as usize] = sdf(p);
    }

    let mut buffer = SurfaceNetsBuffer::default();
    surface_nets(&samples, &SampleShape {}, [0; 3], [33; 3], &mut buffer);

    // Some triangles were generated.
    assert!(!buffer.indices.is_empty());

    let num_vertices = buffer.positions.len();

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(buffer.positions.clone()),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(buffer.normals.clone()),
    );
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices]),
    );
    render_mesh.set_indices(Some(Indices::U32(buffer.indices.clone())));

    (buffer, meshes.add(render_mesh))
}

fn gen_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<MyCustomMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (sphere_buffer, sphere_mesh) = generate_mesh(&mut meshes, |p| sphere_and_cube(0.9, Vec3A::splat(0.7), p));

    // Make a procedural mesh
    commands.spawn(PbrBundle /*MaterialMeshBundle*/ {
        mesh: sphere_mesh,
        transform: Transform::from_xyz(0.0, 0., 0.0).with_scale(Vec3::splat(1.0)),
        /*material: materials.add(MyCustomMaterial {
            color: Color::GREEN,
            time: 0.0,
            color_texture: Some(asset_server.load("block.png")),
            noise_texture: Some(asset_server.load("manifold_noise.png")),
            alpha_mode: AlphaMode::Opaque,
        }*/
        material: materials.add( StandardMaterial {
            base_color:         Color::SEA_GREEN,
            ..default()
        }
        ),
        ..default()
    })
    // Custom components
    .insert(ProcMesh);
}

fn update_mesh(
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    mut lines: ResMut<DebugLines>,
){
    // TODO Edit mesh here....

    // ...

    // Draw bounds
    let size = 32.;
    let bottom_0 = Vec3::splat(0.);
    let bottom_1 = size * Vec3::new(1., 0., 0.);
    let bottom_2 = size * Vec3::new(0., 0., 1.);
    let bottom_3 = size * Vec3::new(1., 0., 1.);
    let top_0 = size * Vec3::new(0., 1., 0.);
    let top_1 = size * Vec3::new(1., 1., 0.);
    let top_2 = size * Vec3::new(0., 1., 1.);
    let top_3 = size * Vec3::new(1., 1., 1.);
    // Bottom
    lines.line_colored(bottom_0, bottom_1, 0., Color::RED);
    lines.line_colored(bottom_0, bottom_2, 0., Color::RED);
    lines.line_colored(bottom_1, bottom_3, 0., Color::RED);
    lines.line_colored(bottom_2, bottom_3, 0., Color::RED);
    // Top
    lines.line_colored(top_0, top_1, 0., Color::RED);
    lines.line_colored(top_0, top_2, 0., Color::RED);
    lines.line_colored(top_1, top_3, 0., Color::RED);
    lines.line_colored(top_2, top_3, 0., Color::RED);
    // Legs
    lines.line_colored(bottom_0, top_0, 0., Color::RED);
    lines.line_colored(bottom_1, top_1, 0., Color::RED);
    lines.line_colored(bottom_2, top_2, 0., Color::RED);
    lines.line_colored(bottom_3, top_3, 0., Color::RED);
}


fn sphere(radius: f32, p: Vec3A) -> f32 {
    p.length() - radius
}

fn into_domain(array_dim: u32, [x, y, z]: [u32; 3]) -> Vec3A {
    (2.0 / array_dim as f32) * Vec3A::new(x as f32, y as f32, z as f32) - 1.0
}

fn cube(b: Vec3A, p: Vec3A) -> f32 {
    let q = p.abs() - b;
    q.max(Vec3A::ZERO).length() + q.max_element().min(0.0)
}

fn link(le: f32, r1: f32, r2: f32, p: Vec3A) -> f32 {
    let q = Vec3A::new(p.x, (p.y.abs() - le).max(0.0), p.z);
    Vec2::new(q.length() - r1, q.z).length() - r2
}

fn sphere_and_cube(s_radius: f32,
                   c_b: Vec3A,
                   p: Vec3A) -> f32 {
    let sphere = p.length() - s_radius;
    let c_q = p.abs() - c_b;
    let cube = c_q.max(Vec3A::ZERO).length() + c_q.max_element().min(0.0);
    sphere.min(cube)
}