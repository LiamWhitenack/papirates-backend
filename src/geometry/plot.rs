use crate::geometry::polyhedron::Polyhedron;
use plotters::{
    backend::SVGBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    element::{Circle, Text},
    series::LineSeries,
    style::{BLACK, Color, IntoFont, RED, WHITE},
};

impl Polyhedron {
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
