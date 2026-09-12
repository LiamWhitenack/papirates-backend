mod geometry;
mod icosahedron;

use crate::icosahedron::icosahedron;

fn main() {
    match icosahedron().plot("icosahedron.svg") {
        Ok(()) => println!("Wrote icosahedron.svg"),
        Err(error) => eprintln!("Failed to write icosahedron.svg: {error}"),
    }
    match icosahedron().dual().plot("dodecahedron.svg") {
        Ok(()) => println!("Wrote dodecahedron.svg"),
        Err(error) => eprintln!("Failed to write dodecahedron.svg: {error}"),
    }
}
