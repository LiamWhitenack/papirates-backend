use std::collections::HashMap;

use crate::geometry::Polyhedron;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
}

impl Edge {
    pub fn new(a: usize, b: usize) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FaceNeighbor {
    pub face: usize,
    pub edge: Edge,
}

#[derive(Debug)]
pub struct Topology {
    pub edges: Vec<Edge>,

    // For each face, the faces adjacent to it.
    pub face_neighbors: Vec<Vec<FaceNeighbor>>,
}

impl Topology {
    pub fn from_polyhedron(polyhedron: &Polyhedron) -> Self {
        let face_count = polyhedron.faces.len();

        let mut edges = Vec::new();

        let mut edge_map: HashMap<Edge, usize> = HashMap::new();

        let mut face_neighbors = vec![Vec::new(); face_count];

        for (face_index, face) in polyhedron.faces.iter().enumerate() {
            let face_edges = [
                Edge::new(face[0], face[1]),
                Edge::new(face[1], face[2]),
                Edge::new(face[2], face[0]),
            ];

            for edge in face_edges {
                let edge_index = if let Some(&index) = edge_map.get(&edge) {
                    index
                } else {
                    let index = edges.len();

                    edges.push(edge);
                    edge_map.insert(edge, index);

                    index
                };

                // If this edge already belongs to another
                // face, then those two faces are neighbors.
                for (other_face, other_edges) in polyhedron.faces.iter().enumerate() {
                    if other_face == face_index {
                        continue;
                    }

                    let other_face_edges = [
                        Edge::new(other_edges[0], other_edges[1]),
                        Edge::new(other_edges[1], other_edges[2]),
                        Edge::new(other_edges[2], other_edges[0]),
                    ];

                    if other_face_edges.contains(&edge)
                        && !face_neighbors[face_index]
                            .iter()
                            .any(|n| n.face == other_face)
                    {
                        face_neighbors[face_index].push(FaceNeighbor {
                            face: other_face,
                            edge,
                        });
                    }
                }

                let _ = edge_index;
            }
        }

        Self {
            edges,
            face_neighbors,
        }
    }
}
