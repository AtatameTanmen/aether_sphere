use super::*;

pub struct IcoSphere {
    mesh: HalfEdgeMesh,
}

impl IcoSphere {
    const A: f64 = 0.5257311121191336;
    const B: f64 = 0.8506508083520399;

    const VERTICES: [F64x3; 12] = [
        F64x3 {
            x: Self::A,
            y: Self::B,
            z: 0.0,
        },
        F64x3 {
            x: -Self::A,
            y: Self::B,
            z: 0.0,
        },
        F64x3 {
            x: -Self::A,
            y: -Self::B,
            z: 0.0,
        },
        F64x3 {
            x: Self::A,
            y: -Self::B,
            z: 0.0,
        },
        F64x3 {
            x: 0.0,
            y: Self::A,
            z: Self::B,
        },
        F64x3 {
            x: 0.0,
            y: -Self::A,
            z: Self::B,
        },
        F64x3 {
            x: 0.0,
            y: -Self::A,
            z: -Self::B,
        },
        F64x3 {
            x: 0.0,
            y: Self::A,
            z: -Self::B,
        },
        F64x3 {
            x: Self::B,
            y: 0.0,
            z: Self::A,
        },
        F64x3 {
            x: Self::B,
            y: 0.0,
            z: -Self::A,
        },
        F64x3 {
            x: -Self::B,
            y: 0.0,
            z: -Self::A,
        },
        F64x3 {
            x: -Self::B,
            y: 0.0,
            z: Self::A,
        },
    ];

    const INDICES: [[usize; 3]; 20] = [
        [0, 1, 4],
        [0, 4, 8],
        [0, 8, 9],
        [0, 9, 7],
        [0, 7, 1],
        [1, 7, 10],
        [1, 10, 11],
        [1, 11, 4],
        [2, 3, 5],
        [2, 5, 11],
        [2, 11, 10],
        [2, 10, 6],
        [2, 6, 3],
        [3, 6, 9],
        [3, 9, 8],
        [3, 8, 5],
        [4, 11, 5],
        [4, 5, 8],
        [6, 10, 7],
        [6, 7, 9],
    ];

    pub fn make_sphere(radius: f64, subdivision_level: usize, lloyd_iterations: usize) -> Self {
        let mut sphere = Self::initial_sphere();
        for _ in 0..subdivision_level {
            sphere.subdivision();
        }
        for _ in 0..lloyd_iterations {
            sphere.lloyd_step();
        }
        sphere.scale(radius);

        sphere
    }

    pub fn into_vertices_indices(&self) -> (Vec<[f32; 3]>, Vec<u32>) {
        let mesh = &self.mesh;
        let vertices: Vec<[f32; 3]> = mesh.vertices.iter().map(|v| v.position.into()).collect();
        let mut indices: Vec<u32> = Vec::with_capacity(mesh.faces.len() * 3);

        for face in mesh.faces.iter() {
            let edge1 = &mesh.half_edges[face.half_edge];
            let edge2 = &mesh.half_edges[edge1.next];
            let edge3 = &mesh.half_edges[edge2.next];

            indices.push(edge1.vertex as u32);
            indices.push(edge2.vertex as u32);
            indices.push(edge3.vertex as u32);
        }

        (vertices, indices)
    }

    pub fn into_triangle_list(&self) -> Vec<[f32; 3]> {
        let mesh = &self.mesh;
        let mut triangle_list: Vec<[f32; 3]> = Vec::with_capacity(mesh.faces.len() * 3);

        for face in mesh.faces.iter() {
            let edge1 = &mesh.half_edges[face.half_edge];
            let edge2 = &mesh.half_edges[edge1.next];
            let edge3 = &mesh.half_edges[edge2.next];

            let pos1 = mesh.vertices[edge1.vertex].position;
            let pos2 = mesh.vertices[edge2.vertex].position;
            let pos3 = mesh.vertices[edge3.vertex].position;

            triangle_list.push(pos1.into());
            triangle_list.push(pos2.into());
            triangle_list.push(pos3.into());
        }

        triangle_list
    }

    fn initial_sphere() -> Self {
        Self {
            mesh: HalfEdgeMesh::from_vertices_indices(&Self::VERTICES, &Self::INDICES),
        }
    }

    fn subdivision(&mut self) {
        let mesh = &mut self.mesh;
        let (old_vertices, old_faces, old_edges) = (&mesh.vertices, &mesh.faces, &mesh.half_edges);
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
                let mid_point = mesh.mid_point(old_edge_index);

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

            faces.push(Face {
                // 0
                half_edge: new_edge_index0,
            });

            faces.push(Face {
                // 1
                half_edge: new_edge_index2,
            });

            faces.push(Face {
                // 2
                half_edge: new_edge_index4,
            });

            faces.push(Face {
                // 3
                half_edge: new_edge_index7,
            });
        }

        *mesh = HalfEdgeMesh {
            vertices,
            faces,
            half_edges,
        };
    }

    fn lloyd_step(&mut self) {
        let mesh = &mut self.mesh;
        let old_vertices = mesh.vertices.clone();
        let vertices = &mut mesh.vertices;
        let faces = &mesh.faces;
        let half_edges = &mesh.half_edges;

        let mut circumcenters: Vec<F64x3> = Vec::with_capacity(faces.len());
        for face in faces.iter() {
            let edge_index0 = face.half_edge;
            let edge_index1 = half_edges[edge_index0].next;
            let edge_index2 = half_edges[edge_index1].next;

            let p0 = old_vertices[half_edges[edge_index0].vertex].position;
            let p1 = old_vertices[half_edges[edge_index1].vertex].position;
            let p2 = old_vertices[half_edges[edge_index2].vertex].position;

            let v1 = p1 - p0;
            let v2 = p2 - p0;

            let v1v1 = v1.dot(v1);
            let v1v2 = v1.dot(v2);
            let v2v2 = v2.dot(v2);

            let divisor = 0.5 / (v1v1 * v2v2 - v1v2 * v1v2);

            let c1 = (v1v1 * v2v2 - v2v2 * v1v2) * divisor;
            let c2 = (-v1v1 * v1v2 + v2v2 * v1v1) * divisor;

            circumcenters.push(p0 + v1 * c1 + v2 * c2);
        }

        for (vertex_index, old_vertex) in old_vertices.iter().enumerate() {
            let p0 = old_vertex.position;
            let mut edge_index = old_vertex.half_edge;
            let mut p1 = circumcenters[half_edges[edge_index].face];
            let mut result: F64x3 = F64x3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let mut area: f64 = 0.0;

            loop {
                edge_index = half_edges[half_edges[edge_index].prev].twin;
                let p2 = circumcenters[half_edges[edge_index].face];

                let v1 = p1 - p0;
                let v2 = p2 - p0;

                let t_area = v1.cross(v2).norm();
                result += (v1 + v2) * t_area;
                area += t_area;

                p1 = p2;
                if edge_index == old_vertex.half_edge {
                    break;
                }
            }

            let divisor = 1.0 / (3.0 * area);
            vertices[vertex_index].position = (p0 + result * divisor).normalize();
        }
    }

    pub fn scale(&mut self, s: f64) {
        for vertex in self.mesh.vertices.iter_mut() {
            vertex.position *= s;
        }
    }
}

impl HalfEdgeMesh {
    fn mid_point(&self, edge_index: usize) -> F64x3 {
        let cur_edge = &self.half_edges[edge_index];
        let next_edge = &self.half_edges[cur_edge.next];
        let point1 = self.vertices[cur_edge.vertex].position;
        let point2 = self.vertices[next_edge.vertex].position;

        (point1 + point2).normalize()
    }
}
