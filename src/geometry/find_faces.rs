use crate::geometry::objects::{Face, Vec3};

pub fn faces_from_vertices(vertices: &[Vec3]) -> Vec<Face> {
    const EPSILON: f64 = 1e-10;

    let mut faces: Vec<Face> = Vec::new();

    for a in 0..vertices.len() {
        for b in (a + 1)..vertices.len() {
            for c in (b + 1)..vertices.len() {
                let ab = vertices[b].sub(vertices[a]);
                let ac = vertices[c].sub(vertices[a]);

                let normal = ab.cross(ac);

                // Degenerate triangle.
                if normal.dot(normal) < EPSILON {
                    continue;
                }

                let mut positive = false;
                let mut negative = false;

                for (i, vertex) in vertices.iter().enumerate() {
                    if i == a || i == b || i == c {
                        continue;
                    }

                    let distance = normal.dot(vertex.sub(vertices[a]));

                    if distance > EPSILON {
                        positive = true;
                    } else if distance < -EPSILON {
                        negative = true;
                    }

                    // The triangle is not on the convex hull.
                    if positive && negative {
                        break;
                    }
                }

                // All other vertices lie on the same side of the plane,
                // so this triangle is a face of the convex hull.
                if !(positive && negative) {
                    faces.push(Face {
                        vertices: vec![a, b, c],
                    });
                }
            }
        }
    }

    faces
}
