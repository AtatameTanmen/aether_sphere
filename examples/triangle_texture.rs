use bevy::{
    asset::RenderAssetUsages, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef,
};

const SHADER_ASSET_PATH: &str = "shaders/example_triangle.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<TriangleMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TriangleMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 30.0,
        }),
    ));

    let vertices: Vec<[f32; 3]> = vec![[-5.0, 5.0, 0.0], [-5.0, -5.0, 0.0], [5.0, 5.0, 0.0]];
    let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 0.0]];

    let mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(TriangleMaterial {})),
        Transform::default(),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct TriangleMaterial {}

impl Material for TriangleMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
