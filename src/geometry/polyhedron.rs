use crate::geometry::{
    find_faces::faces_from_vertices,
    objects::{Face, Vec3},
};

#[derive(Debug, Clone)]
pub struct Polyhedron {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
}
impl Polyhedron {
    pub fn new(vertices: Vec<Vec3>) -> Self {
        let faces = faces_from_vertices(&vertices);

        Self { vertices, faces }
    }

    pub fn dual(&self) -> Polyhedron {
        let new_vertices: Vec<Vec3> = self.faces.iter().map(|face| face.centroid()).collect();

        Polyhedron::new(new_vertices)
    }
}
