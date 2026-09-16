use rstest::rstest;

use crate::geometry::polyhedron::Polyhedron;
#[cfg(test)]
use crate::platonic_solids::PolyhedronTestCase;

impl Polyhedron {
    pub fn dual(&self) -> Polyhedron {
        let topology = self.topology();

        let faces = (0..self.vertices.len())
            .map(|vertex| {
                let incident = self
                    .faces
                    .iter()
                    .enumerate()
                    .filter_map(|(i, face)| face.contains(&vertex).then_some(i))
                    .collect::<Vec<_>>();

                assert!(
                    incident.len() >= 3,
                    "vertex {vertex} belongs to only {} faces: {incident:?}",
                    incident.len()
                );

                let start = incident[0];

                let mut ordered = vec![start];
                let mut previous = None;
                let mut current = start;

                loop {
                    let next = topology[&current]
                        .iter()
                        .copied()
                        .filter(|&neighbor| {
                            incident.contains(&neighbor) && Some(neighbor) != previous
                        })
                        .next();

                    let Some(next) = next else {
                        break;
                    };

                    if next == start {
                        break;
                    }

                    ordered.push(next);
                    previous = Some(current);
                    current = next;
                }

                assert_eq!(
                    ordered.len(),
                    incident.len(),
                    "could not order all faces around vertex {vertex}: \
                 ordered {ordered:?}, incident {incident:?}"
                );

                ordered
            })
            .collect::<Vec<_>>();

        let vertices = self
            .faces
            .iter()
            .map(|face| self.centroid(face))
            .collect::<Vec<_>>();

        Polyhedron::new(vertices, faces)
    }
}

#[rstest]
#[case(crate::platonic_solids::tetrahedron_test_case())]
#[case(crate::platonic_solids::cube_test_case())]
#[case(crate::platonic_solids::octahedron_test_case())]
#[case(crate::platonic_solids::dodecahedron_test_case())]
#[case(crate::platonic_solids::icosahedron_test_case())]
fn test_dual(#[case] polyhedron_test_case: PolyhedronTestCase) {
    let equivalent = polyhedron_test_case
        .polyhedron
        .equivalent(&polyhedron_test_case.polyhedron.dual().dual());
    assert!(equivalent)
}
