use std::collections::HashMap;

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
    pub fn from_triangle_list(vertices: &Vec<[f64; 3]>, indices: &Vec<usize>) -> Self {
        let mut vertices: Vec<Vertex> = vertices
            .iter()
            .map(|&x| Vertex {
                position: x,
                half_edge: INVALID,
            })
            .collect();
        let faces_num = indices.len() / 3;
        let mut faces: Vec<Face> = vec![Face { half_edge: INVALID }; faces_num];
        let mut half_edges: Vec<HalfEdge> = Vec::new();
        let mut edge_map: HashMap<(usize, usize), usize> = HashMap::new();
        let mut face_index: usize = 0;

        while face_index < faces_num {
            let vertex_index0 = indices[3 * face_index];
            let vertex_index1 = indices[3 * face_index + 1];
            let vertex_index2 = indices[3 * face_index + 2];
            let edge_index0 = half_edges.len();
            let edge_index1 = edge_index0 + 1;
            let edge_index2 = edge_index0 + 2;

            faces[face_index].half_edge = edge_index0;

            half_edges.push(HalfEdge {
                vertex: vertex_index0,
                face: face_index,
                next: edge_index1,
                prev: edge_index2,
                twin: INVALID,
            });
            half_edges.push(HalfEdge {
                vertex: vertex_index1,
                face: face_index,
                next: edge_index2,
                prev: edge_index0,
                twin: INVALID,
            });
            half_edges.push(HalfEdge {
                vertex: vertex_index2,
                face: face_index,
                next: edge_index0,
                prev: edge_index1,
                twin: INVALID,
            });

            if vertices[vertex_index0].half_edge == INVALID {
                vertices[vertex_index0].half_edge = edge_index0;
            }
            if vertices[vertex_index1].half_edge == INVALID {
                vertices[vertex_index1].half_edge = edge_index1;
            }
            if vertices[vertex_index2].half_edge == INVALID {
                vertices[vertex_index2].half_edge = edge_index2;
            }

            if let Some(&twin) = edge_map.get(&(vertex_index1, vertex_index0)) {
                half_edges[edge_index0].twin = twin;
                half_edges[twin].twin = edge_index0;
            } else {
                edge_map.insert((vertex_index0, vertex_index1), edge_index0);
            }
            if let Some(&twin) = edge_map.get(&(vertex_index2, vertex_index1)) {
                half_edges[edge_index1].twin = twin;
                half_edges[twin].twin = edge_index1;
            } else {
                edge_map.insert((vertex_index1, vertex_index2), edge_index1);
            }
            if let Some(&twin) = edge_map.get(&(vertex_index0, vertex_index2)) {
                half_edges[edge_index2].twin = twin;
                half_edges[twin].twin = edge_index2;
            } else {
                edge_map.insert((vertex_index2, vertex_index0), edge_index2);
            }

            face_index += 1;
        }

        Self {
            vertices,
            faces,
            half_edges,
        }
    }
}
