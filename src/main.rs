mod geometry;
mod platonic_solids;

use crate::platonic_solids::icosahedron;

fn main() {
    match icosahedron().plot("icosahedron.svg") {
        Ok(()) => println!("Wrote icosahedron.svg"),
        Err(error) => eprintln!("Failed to write icosahedron.svg: {error}"),
    }
    match icosahedron().dual().plot("dodecahedron.svg") {
        Ok(()) => println!("Wrote dodecahedron.svg"),
        Err(error) => eprintln!("Failed to write dodecahedron.svg: {error}"),
    }
    match icosahedron().dual().dual().plot("isocahedron II.svg") {
        Ok(()) => println!("Wrote isocahedron II.svg"),
        Err(error) => eprintln!("Failed to write isocahedron II.svg: {error}"),
    }
}
