use std::collections::HashMap;

use glam::Vec3;
use plotters::{
    backend::SVGBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    element::{Circle, Text},
    series::LineSeries,
    style::{BLACK, Color, IntoFont, RED, WHITE},
};

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

    pub fn topology(&self) -> HashMap<usize, Vec<usize>> {
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

    pub fn plot(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let root = SVGBackend::new(path, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE)?;

        let range = self
            .vertices
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .fold(0.0_f32, |max, value| max.max(value.abs())) as f64;

        let mut chart = ChartBuilder::on(&root).margin(20).build_cartesian_3d(
            -range..range,
            -range..range,
            -range..range,
        )?;

        chart.configure_axes().draw()?;

        // Draw edges.
        let mut edges = std::collections::HashSet::new();

        for face in &self.faces {
            for i in 0..face.len() {
                let a = face[i];
                let b = face[(i + 1) % face.len()];
                edges.insert((a.min(b), a.max(b)));
            }
        }

        for (a, b) in edges {
            let a = self.vertices[a];
            let b = self.vertices[b];

            chart.draw_series(LineSeries::new(
                vec![
                    (a.x as f64, a.y as f64, a.z as f64),
                    (b.x as f64, b.y as f64, b.z as f64),
                ],
                &BLACK,
            ))?;
        }

        // Draw vertices.
        chart.draw_series(self.vertices.iter().map(|vertex| {
            Circle::new(
                (vertex.x as f64, vertex.y as f64, vertex.z as f64),
                5,
                RED.filled(),
            )
        }))?;

        // Draw vertex indices.
        chart.draw_series(self.vertices.iter().enumerate().map(|(index, vertex)| {
            Text::new(
                index.to_string(),
                (vertex.x as f64, vertex.y as f64, vertex.z as f64),
                ("sans-serif", 20).into_font(),
            )
        }))?;

        root.present()?;

        Ok(())
    }
}
