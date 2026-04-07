mod game;
mod half_edge_mesh;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            game::camera::CameraPlugin,
            game::mesh::MeshPlugin,
        ))
        .run();
    // let vertices: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    // let indices: Vec<usize> = vec![0, 1, 2, 2, 1, 0];

    // println!("{:#?}", half_edge_mesh::HalfEdgeMesh::from_triangle_list(&vertices, &indices));
}
