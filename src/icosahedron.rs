use glam::Vec3;

use crate::geometry::polyhedron::Polyhedron;

pub fn icosahedron() -> Polyhedron {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;

    let vertices = vec![
        Vec3::new(0.0, 1.0, phi),   // 0
        Vec3::new(0.0, -1.0, phi),  // 1
        Vec3::new(0.0, 1.0, -phi),  // 2
        Vec3::new(0.0, -1.0, -phi), // 3
        Vec3::new(1.0, phi, 0.0),   // 4
        Vec3::new(-1.0, phi, 0.0),  // 5
        Vec3::new(1.0, -phi, 0.0),  // 6
        Vec3::new(-1.0, -phi, 0.0), // 7
        Vec3::new(phi, 0.0, 1.0),   // 8
        Vec3::new(phi, 0.0, -1.0),  // 9
        Vec3::new(-phi, 0.0, 1.0),  // 10
        Vec3::new(-phi, 0.0, -1.0), // 11
    ];

    let faces = vec![
        vec![0, 1, 8],
        vec![0, 8, 4],
        vec![0, 4, 5],
        vec![0, 5, 10],
        vec![0, 10, 1],
        vec![1, 10, 7],
        vec![1, 7, 6],
        vec![1, 6, 8],
        vec![2, 3, 9],
        vec![2, 9, 4],
        vec![2, 4, 5],
        vec![2, 5, 11],
        vec![2, 11, 3],
        vec![3, 11, 7],
        vec![3, 7, 6],
        vec![3, 6, 9],
        vec![4, 9, 8],
        vec![10, 5, 11],
        vec![10, 11, 7],
        vec![8, 6, 9],
    ];

    Polyhedron { vertices, faces }
}
