pub mod ico_sphere;

use bevy::platform::collections::HashMap;

const INVALID: usize = usize::MAX;

#[derive(Debug)]
struct Vertex {
    position: [f64; 3],
    half_edge: usize,
}

#[derive(Debug, Clone)]
struct Face {
    half_edge: usize,
}

#[derive(Debug)]
struct HalfEdge {
    vertex: usize,
    face: usize,
    next: usize,
    prev: usize,
    twin: usize,
}

#[derive(Debug)]
pub struct HalfEdgeMesh {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    half_edges: Vec<HalfEdge>,
}

impl HalfEdgeMesh {
    pub fn from_vertices_indices(vertices: &Vec<[f64; 3]>, indices: &Vec<usize>) -> Self {
        let mut vertices: Vec<Vertex> = vertices
            .iter()
            .map(|&x| Vertex {
                position: x,
                half_edge: INVALID,
            })
            .collect();
        let faces_num = indices.len() / 3;
        let mut faces: Vec<Face> = vec![Face { half_edge: INVALID }; faces_num];
        let mut half_edges: Vec<HalfEdge> = Vec::with_capacity(3 * faces_num);
        let mut edge_map: HashMap<(usize, usize), usize> = HashMap::new();

        for (face_index, vertex_indices) in indices.as_chunks::<3>().0.iter().enumerate() {
            let face_index3 = 3 * face_index;
            let edge_indices = [face_index3, face_index3 + 1, face_index3 + 2];

            faces[face_index].half_edge = edge_indices[0];

            half_edges.push(HalfEdge {
                vertex: vertex_indices[0],
                face: face_index,
                next: edge_indices[1],
                prev: edge_indices[2],
                twin: INVALID,
            });
            half_edges.push(HalfEdge {
                vertex: vertex_indices[1],
                face: face_index,
                next: edge_indices[2],
                prev: edge_indices[0],
                twin: INVALID,
            });
            half_edges.push(HalfEdge {
                vertex: vertex_indices[2],
                face: face_index,
                next: edge_indices[0],
                prev: edge_indices[1],
                twin: INVALID,
            });

            vertices[vertex_indices[0]].half_edge = edge_indices[0];
            vertices[vertex_indices[1]].half_edge = edge_indices[1];
            vertices[vertex_indices[2]].half_edge = edge_indices[2];

            if let Some(&twin0) = edge_map.get(&(vertex_indices[1], vertex_indices[0])) {
                half_edges[edge_indices[0]].twin = twin0;
                half_edges[twin0].twin = edge_indices[0];
            } else {
                edge_map.insert((vertex_indices[0], vertex_indices[1]), edge_indices[0]);
            }
            if let Some(&twin1) = edge_map.get(&(vertex_indices[2], vertex_indices[1])) {
                half_edges[edge_indices[1]].twin = twin1;
                half_edges[twin1].twin = edge_indices[1];
            } else {
                edge_map.insert((vertex_indices[1], vertex_indices[2]), edge_indices[1]);
            }
            if let Some(&twin2) = edge_map.get(&(vertex_indices[0], vertex_indices[2])) {
                half_edges[edge_indices[2]].twin = twin2;
                half_edges[twin2].twin = edge_indices[2];
            } else {
                edge_map.insert((vertex_indices[2], vertex_indices[0]), edge_indices[2]);
            }
        }

        Self {
            vertices,
            faces,
            half_edges,
        }
    }

    /// 完全に閉じたメッシュを正しく表している場合、真を返す
    pub fn check(&self) -> bool {
        let edges_num = self.half_edges.len();
        for (edge_index, edge) in self.half_edges.iter().enumerate() {
            if edge.twin >= edges_num
                || edge.next >= edges_num
                || self.half_edges[edge.twin].next >= edges_num
                || {
                    let twin = &self.half_edges[edge.twin];
                    let next = &self.half_edges[edge.next];
                    let twin_next = &self.half_edges[twin.next];

                    twin.twin != edge_index
                        || next.prev != edge_index
                        || next.face != edge.face
                        || twin_next.vertex != edge.vertex
                }
            {
                return false;
            }
        }

        for (face_index, face) in self.faces.iter().enumerate() {
            if face.half_edge >= edges_num || self.half_edges[face.half_edge].face != face_index {
                return false;
            }
        }

        for (vertex_index, vertex) in self.vertices.iter().enumerate() {
            if vertex.half_edge >= edges_num || self.half_edges[vertex.half_edge].vertex != vertex_index {
                return false;
            }
        }
        
        return true;
    }
}
