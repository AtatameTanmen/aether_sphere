mod half_edge_mesh;

fn main() {
    let vertices: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let triangles: Vec<usize> = vec![0, 1, 2, 2, 1, 0];

    println!("{:#?}", half_edge_mesh::HalfEdgeMesh::from_triangle_list(&vertices, &triangles));
}
