Before we do anything else, it seems best to make a way to visualize and create the map structure in the backend. 

We are trying to make a [Goldberg Polyhedron](https://en.wikipedia.org/wiki/Goldberg_polyhedron) of the class II varietry, meaning that we need to implement the following methods:
- Make an icosahedron
- Dual
- Truncate
- Map onto a sphere shape (skip; probably only in the front end)

Because we want to be able to easily change the map shape, we need to be able to serialize the coordinates and index each cell and its neighbors. The easiest way to do this would probably be using each shape's 