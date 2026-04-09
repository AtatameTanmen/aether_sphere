use crate::half_edge_mesh::{Face, HalfEdge, HalfEdgeMesh, INVALID, Vertex};

const A: f64 = 0.5257311121191336;
const B: f64 = 0.8506508083520399;

pub const VERTICES: [[f64; 3]; 12] = [
    [A, B, 0.0],
    [-A, B, 0.0],
    [-A, -B, 0.0],
    [A, -B, 0.0],
    [0.0, A, B],
    [0.0, -A, B],
    [0.0, -A, -B],
    [0.0, A, -B],
    [B, 0.0, A],
    [B, 0.0, -A],
    [-B, 0.0, -A],
    [-B, 0.0, A],
];

pub const INDICES: [usize; 60] = [
    0, 1, 4, 0, 4, 8, 0, 8, 9, 0, 9, 7, 0, 7, 1, 1, 7, 10, 1, 10, 11, 1, 11, 4, 2, 3, 5, 2, 5, 11,
    2, 11, 10, 2, 10, 6, 2, 6, 3, 3, 6, 9, 3, 9, 8, 3, 8, 5, 4, 11, 5, 4, 5, 8, 6, 10, 7, 6, 7, 9,
];

pub fn make_ico_sphere(n: usize) -> HalfEdgeMesh {
    let mut sphere = HalfEdgeMesh::from_vertices_indices(&VERTICES.to_vec(), &INDICES.to_vec());
    for _ in 0..n {
        let (old_vertices, old_faces, old_edges) =
            (&sphere.vertices, &sphere.faces, &sphere.half_edges);
        let old_vertices_num = old_vertices.len();
        let old_faces_num = old_faces.len();
        let old_edges_num = old_edges.len();

        let mut vertices: Vec<Vertex> = Vec::with_capacity(old_vertices_num + (old_edges_num >> 1));
        let mut faces: Vec<Face> = Vec::with_capacity(old_faces_num << 2);
        let mut half_edges: Vec<HalfEdge> = Vec::with_capacity(old_edges_num << 2);

        vertices.append(
            &mut old_vertices
                .iter()
                .map(|x| Vertex {
                    position: x.position,
                    half_edge: x.half_edge << 1,
                })
                .collect(),
        );

        for old_edge_index in 0..old_edges_num {
            let new_edge_index0 = half_edges.len();
            let new_edge_index1 = new_edge_index0 + 1;
            half_edges.push(HalfEdge {
                vertex: old_edges[old_edge_index].vertex,
                face: INVALID,
                next: INVALID,
                prev: INVALID,
                twin: INVALID,
            });
            half_edges.push(HalfEdge {
                vertex: INVALID,
                face: INVALID,
                next: INVALID,
                prev: INVALID,
                twin: INVALID,
            });

            if old_edges[old_edge_index].twin < old_edge_index {
                let twin_edge_index0 = old_edges[old_edge_index].twin << 1;
                let twin_edge_index1 = twin_edge_index0 + 1;
                let mid_vertex_index = half_edges[twin_edge_index1].vertex;

                half_edges[new_edge_index0].twin = twin_edge_index1;
                half_edges[new_edge_index1].twin = twin_edge_index0;
                half_edges[twin_edge_index0].twin = new_edge_index1;
                half_edges[twin_edge_index1].twin = new_edge_index0;

                half_edges[new_edge_index1].vertex = mid_vertex_index;
            } else {
                let mid_vertex_index = vertices.len();
                let mid_point = sphere.mid_point(old_edge_index);

                vertices.push(Vertex {
                    position: mid_point,
                    half_edge: new_edge_index1,
                });
                half_edges[new_edge_index1].vertex = mid_vertex_index;
            }
        }

        for old_face_index in 0..old_faces_num {
            let old_edge_index0 = old_faces[old_face_index].half_edge;
            let old_edge_index1 = old_edges[old_edge_index0].next;
            let old_edge_index2 = old_edges[old_edge_index1].next;

            let new_edge_index0 = old_edge_index0 << 1;
            let new_edge_index1 = new_edge_index0 + 1;
            let new_edge_index2 = old_edge_index1 << 1;
            let new_edge_index3 = new_edge_index2 + 1;
            let new_edge_index4 = old_edge_index2 << 1;
            let new_edge_index5 = new_edge_index4 + 1;
            let new_edge_index6 = half_edges.len();
            let new_edge_index7 = new_edge_index6 + 1;
            let new_edge_index8 = new_edge_index6 + 2;
            let new_edge_index9 = new_edge_index6 + 3;
            let new_edge_index10 = new_edge_index6 + 4;
            let new_edge_index11 = new_edge_index6 + 5;

            let mid_vertex_index0 = half_edges[new_edge_index1].vertex;
            let mid_vertex_index1 = half_edges[new_edge_index3].vertex;
            let mid_vertex_index2 = half_edges[new_edge_index5].vertex;

            let new_face_index0 = faces.len();
            let new_face_index1 = new_face_index0 + 1;
            let new_face_index2 = new_face_index0 + 2;
            let new_face_index3 = new_face_index0 + 3;

            half_edges[new_edge_index0].face = new_face_index0;
            half_edges[new_edge_index0].next = new_edge_index6;
            half_edges[new_edge_index0].prev = new_edge_index5;

            half_edges[new_edge_index1].face = new_face_index1;
            half_edges[new_edge_index1].next = new_edge_index2;
            half_edges[new_edge_index1].prev = new_edge_index8;

            half_edges[new_edge_index2].face = new_face_index1;
            half_edges[new_edge_index2].next = new_edge_index8;
            half_edges[new_edge_index2].prev = new_edge_index1;

            half_edges[new_edge_index3].face = new_face_index2;
            half_edges[new_edge_index3].next = new_edge_index4;
            half_edges[new_edge_index3].prev = new_edge_index10;

            half_edges[new_edge_index4].face = new_face_index2;
            half_edges[new_edge_index4].next = new_edge_index10;
            half_edges[new_edge_index4].prev = new_edge_index3;

            half_edges[new_edge_index5].face = new_face_index0;
            half_edges[new_edge_index5].next = new_edge_index0;
            half_edges[new_edge_index5].prev = new_edge_index6;

            half_edges.push(HalfEdge {
                // 6
                vertex: mid_vertex_index0,
                face: new_face_index0,
                next: new_edge_index5,
                prev: new_edge_index0,
                twin: new_edge_index11,
            });

            half_edges.push(HalfEdge {
                // 7
                vertex: mid_vertex_index0,
                face: new_face_index3,
                next: new_edge_index9,
                prev: new_edge_index11,
                twin: new_edge_index8,
            });

            half_edges.push(HalfEdge {
                // 8
                vertex: mid_vertex_index1,
                face: new_face_index1,
                next: new_edge_index1,
                prev: new_edge_index2,
                twin: new_edge_index7,
            });

            half_edges.push(HalfEdge {
                // 9
                vertex: mid_vertex_index1,
                face: new_face_index3,
                next: new_edge_index11,
                prev: new_edge_index7,
                twin: new_edge_index10,
            });

            half_edges.push(HalfEdge {
                // 10
                vertex: mid_vertex_index2,
                face: new_face_index2,
                next: new_edge_index3,
                prev: new_edge_index4,
                twin: new_edge_index9,
            });

            half_edges.push(HalfEdge {
                // 11
                vertex: mid_vertex_index2,
                face: new_face_index3,
                next: new_edge_index7,
                prev: new_edge_index9,
                twin: new_edge_index6,
            });

            faces.push(Face { // 0
                half_edge: new_edge_index0,
            });

            faces.push(Face { // 1
                half_edge: new_edge_index2,
            });

            faces.push(Face { // 2
                half_edge: new_edge_index4,
            });

            faces.push(Face { // 3
                half_edge: new_edge_index7,
            });
        }

        sphere = HalfEdgeMesh {
            vertices,
            faces,
            half_edges,
        };
    }

    sphere
}

impl HalfEdgeMesh {
    fn mid_point(&self, edge_index: usize) -> [f64; 3] {
        let cur_edge = &self.half_edges[edge_index];
        let next_edge = &self.half_edges[cur_edge.next];
        let point1 = self.vertices[cur_edge.vertex].position;
        let point2 = self.vertices[next_edge.vertex].position;

        normalize([
            (point1[0] + point2[0]),
            (point1[1] + point2[1]),
            (point1[2] + point2[2]),
        ])
    }
}

fn normalize(mut point: [f64; 3]) -> [f64; 3] {
    let length = (point[0].powi(2) + point[1].powi(2) + point[2].powi(2)).sqrt();
    point.iter_mut().for_each(|x| *x /= length);
    point
}
