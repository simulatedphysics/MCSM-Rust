use lattice::{Lattice, Site};

struct Square {
    lat: Vec<Site>
}

impl Lattice for Square {
    fn new(n_x: i32, n_y: i32) -> Square {
        let mut lat: Vec<Site> = Vec::new();
        for j in 0..n_y {
            for i in 0..n_x {
                lat.push(Site { x: i as f64, y: j as f64, z: 0.0 });
            }
        }
        Square { lat }
    }

//    fn get_neighbors() -> Vec<Site>{
//
//    }

//    fn get_neighbors(m_x:i32, m_y:i32, n_x:i32, n_y:i32) -> Vec<Site>{
//        let mut neighbors:Vec<Site> = Vec::new();
//        neighbors.push(Site{x:((m_x + 1 + n_x)%n_x) as f64, y:m_y as f64, z:0.0});
//        neighbors.push(Site{x:m_x as f64, y:((m_y + 1 + n_y)%n_y) as f64, z:0.0});
//        neighbors
//    }
}