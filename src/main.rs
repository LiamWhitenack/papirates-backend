mod geometry;
mod isocahedron;
mod plot;

use crate::{isocahedron::icosahedron, plot::plot_2d};

fn main() {
    let polyhedron = icosahedron().dual();

    match plot_2d(&polyhedron, "goldberg.svg") {
        Ok(()) => println!("Wrote goldberg.svg"),
        Err(error) => eprintln!("Failed to write goldberg.svg: {error}"),
    }
}
