use crate::geometry::{
    find_faces::faces_from_vertices,
    objects::{Polyhedron, Vec3},
};

pub fn icosahedron() -> Polyhedron {
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

    let vertices = vec![
        Vec3::new(0.0, 1.0, phi),
        Vec3::new(0.0, -1.0, phi),
        Vec3::new(0.0, 1.0, -phi),
        Vec3::new(0.0, -1.0, -phi),
        Vec3::new(1.0, phi, 0.0),
        Vec3::new(-1.0, phi, 0.0),
        Vec3::new(1.0, -phi, 0.0),
        Vec3::new(-1.0, -phi, 0.0),
        Vec3::new(phi, 0.0, 1.0),
        Vec3::new(phi, 0.0, -1.0),
        Vec3::new(-phi, 0.0, 1.0),
        Vec3::new(-phi, 0.0, -1.0),
    ];

    let faces = faces_from_vertices(&vertices);

    Polyhedron { vertices, faces }
}
