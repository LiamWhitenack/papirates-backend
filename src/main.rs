mod geometry;
mod isocahedron;
mod plot;

use crate::{isocahedron::icosahedron, plot::plot_2d};

fn main() {
    let polyhedron = icosahedron();

    match plot_2d(&polyhedron, "icosahedron.svg") {
        Ok(()) => println!("Wrote icosahedron.svg"),
        Err(error) => eprintln!("Failed to write icosahedron.svg: {error}"),
    }
}
