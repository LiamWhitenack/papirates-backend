use std::collections::HashMap;

use glam::Vec3;

use crate::geometry::polyhedron::Polyhedron;

impl Polyhedron {
    pub fn truncate(&self) -> Self {
        let mut vertices = Vec::new();
        let mut edge_vertices = HashMap::new();

        for face in &self.faces {
            for i in 0..face.len() {
                let a = face[i];
                let b = face[(i + 1) % face.len()];

                edge_vertices.entry((a, b)).or_insert_with(|| {
                    let index = vertices.len();

                    vertices.push(self.vertices[a] * (2.0 / 3.0) + self.vertices[b] * (1.0 / 3.0));

                    index
                });
            }
        }

        let mut faces = Vec::new();

        // Truncated versions of the original faces.
        for face in &self.faces {
            let mut truncated = Vec::with_capacity(face.len() * 2);

            for i in 0..face.len() {
                let a = face[i];
                let b = face[(i + 1) % face.len()];

                truncated.push(edge_vertices[&(a, b)]);
                truncated.push(edge_vertices[&(b, a)]);
            }

            faces.push(truncated);
        }

        // New faces corresponding to the original vertices.
        for vertex in 0..self.vertices.len() {
            let center = self.vertices[vertex];

            let mut surrounding = self
                .faces
                .iter()
                .flat_map(|face| {
                    (0..face.len()).filter_map(|i| {
                        if face[i] != vertex {
                            return None;
                        }

                        let previous = face[(i + face.len() - 1) % face.len()];
                        let next = face[(i + 1) % face.len()];

                        Some([
                            edge_vertices[&(previous, vertex)],
                            edge_vertices[&(vertex, next)],
                        ])
                    })
                })
                .flatten()
                .collect::<Vec<_>>();

            surrounding.sort_unstable();
            surrounding.dedup();

            // Order cyclically around the original vertex.
            let normal = center.normalize();

            let reference = if normal.x.abs() < 0.9 {
                Vec3::new(1.0, 0.0, 0.0)
            } else {
                Vec3::new(0.0, 1.0, 0.0)
            };

            let tangent = normal.cross(reference).normalize();
            let bitangent = normal.cross(tangent).normalize();

            surrounding.sort_by(|&a, &b| {
                let a = vertices[a] - center;
                let b = vertices[b] - center;

                let angle_a = a.dot(bitangent).atan2(a.dot(tangent));
                let angle_b = b.dot(bitangent).atan2(b.dot(tangent));

                angle_a.total_cmp(&angle_b)
            });

            faces.push(surrounding);
        }

        Self::new(vertices, faces)
    }
}
