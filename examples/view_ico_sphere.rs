use aether_sphere::mesh::half_edge_mesh::ico_sphere::*;
use bevy::{
    asset::RenderAssetUsages,
    mesh::Indices,
    pbr::wireframe::{Wireframe, WireframeColor, WireframePlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 30.0,
        }),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::default().looking_to(vec3(-4.0, -3.0, -1.0), vec3(0.0, 1.0, 0.0)),
    ));

    let sphere = IcoSphere::make_sphere(10.0, 6, 30);
    let (vertices, indices) = sphere.into_vertices_indices();

    let mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(indices))
    .with_computed_normals();

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::default(),
        Wireframe,
        WireframeColor {
            color: Color::BLACK,
        },
    ));
}

fn rotate(mut mesh: Single<&mut Transform, With<Mesh3d>>) {
    mesh.rotate(Quat::from_axis_angle(
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        0.001,
    ));
}
