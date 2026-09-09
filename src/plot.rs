use crate::geometry::objects::Polyhedron;
use plotters::prelude::*;

pub fn plot_2d(polyhedron: &Polyhedron, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(path, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Simple orthographic projection: ignore Z.
    let points: Vec<(f64, f64)> = polyhedron.vertices.iter().map(|v| (v.x, v.y)).collect();

    let (min_x, max_x) = points
        .iter()
        .map(|(x, _)| *x)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
            (min.min(x), max.max(x))
        });

    let (min_y, max_y) = points
        .iter()
        .map(|(_, y)| *y)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
            (min.min(y), max.max(y))
        });

    let padding = 0.1;

    let x_padding = (max_x - min_x) * padding;
    let y_padding = (max_y - min_y) * padding;

    let mut chart = ChartBuilder::on(&root).margin(20).build_cartesian_2d(
        min_x - x_padding..max_x + x_padding,
        min_y - y_padding..max_y + y_padding,
    )?;

    chart.configure_mesh().disable_mesh().draw()?;

    for face in &polyhedron.faces {
        let polygon: Vec<(f64, f64)> = face.vertices.iter().map(|&index| points[index]).collect();

        chart.draw_series(std::iter::once(Polygon::new(
            polygon.clone(),
            BLUE.mix(0.2).filled(),
        )))?;

        chart.draw_series(LineSeries::new(
            polygon.iter().copied().chain(std::iter::once(polygon[0])),
            &BLACK,
        ))?;
    }

    chart.draw_series(
        points
            .iter()
            .map(|&point| Circle::new(point, 4, BLACK.filled())),
    )?;

    root.present()?;

    Ok(())
}
