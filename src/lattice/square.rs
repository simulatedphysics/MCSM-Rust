use lattice::{Lattice};

struct Square{

}

impl Lattice for Square {
    fn new(n_x: i32, n_y: i32) -> Lattice {
        let mut lat: Vec<Site> = Vec::new();
        for j in 0..n_y {
            for i in 0..n_x {
                lat.push(Site { x: i as f64, y: j as f64, z: 0.0 });
            }
        }
        Lattice { lattice: lat, primitive_vectors: Vec::new(), basis_vectors: Vec::new() }
    }
}