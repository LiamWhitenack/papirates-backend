use std::collections::HashMap;

use crate::geometry::polyhedron::Polyhedron;

impl Polyhedron {
    pub fn topology(&self) -> HashMap<usize, Vec<usize>> {
        // neighbor schema, index of a face to the index of its neighboring faces
        self.faces
            .iter()
            .enumerate()
            .map(|(i, face)| {
                let neighbors = self
                    .faces
                    .iter()
                    .enumerate()
                    .filter_map(|(j, other)| {
                        (i != j && face.iter().filter(|v| other.contains(v)).count() == 2)
                            .then_some(j)
                    })
                    .collect();

                (i, neighbors)
            })
            .collect()
    }
}
