use crate::geometry::polyhedron::Polyhedron;
use glam::Vec3;
use rstest::fixture;

pub fn tetrahedron() -> Polyhedron {
    Polyhedron::new(
        vec![
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(-1.0, -1.0, 1.0),
        ],
        vec![vec![0, 1, 2], vec![0, 3, 1], vec![0, 2, 3], vec![1, 3, 2]],
    )
}

pub fn cube() -> Polyhedron {
    Polyhedron::new(
        vec![
            Vec3::new(-1.0, -1.0, -1.0), // 0
            Vec3::new(1.0, -1.0, -1.0),  // 1
            Vec3::new(1.0, 1.0, -1.0),   // 2
            Vec3::new(-1.0, 1.0, -1.0),  // 3
            Vec3::new(-1.0, -1.0, 1.0),  // 4
            Vec3::new(1.0, -1.0, 1.0),   // 5
            Vec3::new(1.0, 1.0, 1.0),    // 6
            Vec3::new(-1.0, 1.0, 1.0),   // 7
        ],
        vec![
            vec![0, 1, 2, 3],
            vec![4, 7, 6, 5],
            vec![0, 4, 5, 1],
            vec![1, 5, 6, 2],
            vec![2, 6, 7, 3],
            vec![3, 7, 4, 0],
        ],
    )
}

pub fn octahedron() -> Polyhedron {
    Polyhedron::new(
        vec![
            Vec3::new(1.0, 0.0, 0.0),  // 0
            Vec3::new(-1.0, 0.0, 0.0), // 1
            Vec3::new(0.0, 1.0, 0.0),  // 2
            Vec3::new(0.0, -1.0, 0.0), // 3
            Vec3::new(0.0, 0.0, 1.0),  // 4
            Vec3::new(0.0, 0.0, -1.0), // 5
        ],
        vec![
            vec![0, 2, 4],
            vec![2, 1, 4],
            vec![1, 3, 4],
            vec![3, 0, 4],
            vec![2, 0, 5],
            vec![1, 2, 5],
            vec![3, 1, 5],
            vec![0, 3, 5],
        ],
    )
}

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

pub fn dodecahedron() -> Polyhedron {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let inv_phi = 1.0 / phi;

    let vertices = vec![
        // (±1, ±1, ±1)
        Vec3::new(-1.0, -1.0, -1.0), // 0
        Vec3::new(-1.0, -1.0, 1.0),  // 1
        Vec3::new(-1.0, 1.0, -1.0),  // 2
        Vec3::new(-1.0, 1.0, 1.0),   // 3
        Vec3::new(1.0, -1.0, -1.0),  // 4
        Vec3::new(1.0, -1.0, 1.0),   // 5
        Vec3::new(1.0, 1.0, -1.0),   // 6
        Vec3::new(1.0, 1.0, 1.0),    // 7
        // (0, ±1/φ, ±φ)
        Vec3::new(0.0, -inv_phi, -phi), // 8
        Vec3::new(0.0, -inv_phi, phi),  // 9
        Vec3::new(0.0, inv_phi, -phi),  // 10
        Vec3::new(0.0, inv_phi, phi),   // 11
        // (±1/φ, ±φ, 0)
        Vec3::new(-inv_phi, -phi, 0.0), // 12
        Vec3::new(-inv_phi, phi, 0.0),  // 13
        Vec3::new(inv_phi, -phi, 0.0),  // 14
        Vec3::new(inv_phi, phi, 0.0),   // 15
        // (±φ, 0, ±1/φ)
        Vec3::new(-phi, 0.0, -inv_phi), // 16
        Vec3::new(-phi, 0.0, inv_phi),  // 17
        Vec3::new(phi, 0.0, -inv_phi),  // 18
        Vec3::new(phi, 0.0, inv_phi),   // 19
    ];

    let faces = vec![
        vec![0, 8, 10, 2, 16],
        vec![0, 4, 14, 12, 8],
        vec![0, 16, 17, 1, 12],
        vec![1, 17, 3, 11, 9],
        vec![1, 9, 5, 19, 17],
        vec![2, 10, 6, 15, 13],
        vec![2, 13, 3, 17, 16],
        vec![3, 13, 15, 7, 11],
        vec![4, 18, 6, 10, 8],
        vec![4, 5, 19, 18, 14],
        vec![5, 9, 11, 7, 19],
        vec![6, 18, 19, 7, 15],
    ];

    Polyhedron { vertices, faces }
}

pub struct PolyhedronTestCase {
    pub name: &'static str,
    pub polyhedron: Polyhedron,
    pub vertices: usize,
    pub edges: usize,
    pub faces: usize,
}
#[fixture]
pub fn tetrahedron_test_case() -> PolyhedronTestCase {
    PolyhedronTestCase {
        name: "tetrahedron",
        polyhedron: tetrahedron(),
        vertices: 4,
        edges: 6,
        faces: 4,
    }
}
#[fixture]
pub fn cube_test_case() -> PolyhedronTestCase {
    PolyhedronTestCase {
        name: "cube",
        polyhedron: cube(),
        vertices: 8,
        edges: 12,
        faces: 6,
    }
}
#[fixture]
pub fn octahedron_test_case() -> PolyhedronTestCase {
    PolyhedronTestCase {
        name: "octahedron",
        polyhedron: octahedron(),
        vertices: 6,
        edges: 12,
        faces: 8,
    }
}
#[fixture]
pub fn dodecahedron_test_case() -> PolyhedronTestCase {
    PolyhedronTestCase {
        name: "dodecahedron",
        polyhedron: dodecahedron(),
        vertices: 20,
        edges: 30,
        faces: 12,
    }
}
#[fixture]
pub fn icosahedron_test_case() -> PolyhedronTestCase {
    PolyhedronTestCase {
        name: "icosahedron",
        polyhedron: icosahedron(),
        vertices: 12,
        edges: 30,
        faces: 20,
    }
}
