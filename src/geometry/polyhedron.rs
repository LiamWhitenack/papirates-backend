use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Polyhedron {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Vec<usize>>,
}
impl Polyhedron {
    pub fn new(vertices: Vec<Vec3>, faces: Vec<Vec<usize>>) -> Self {
        Self { vertices, faces }
    }

    pub fn centroid(&self, face: &[usize]) -> Vec3 {
        face.iter().map(|&index| self.vertices[index]).sum::<Vec3>() / face.len() as f32
    }
}
