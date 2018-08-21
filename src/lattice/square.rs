use lattice::{Lattice, Site};

pub struct Square {
    pub lat: Vec<Site>
}

impl Square{
    pub fn map_to_index(m_x:i32, m_y:i32, n_x:i32) -> i32{
        m_y*n_x + m_x
    }
}

impl Square{
    pub fn map_to_site(index:i32, n_x:i32) -> Site{
        Site{x: (index - n_x*(index/n_x)) as f64, y:(index/n_x) as f64, z:0.0, occupant:None}
    }
}

impl Lattice for Square {
    fn new(n_x: i32, n_y: i32) -> Square {
        let mut lat: Vec<Site> = Vec::new();
        for j in 0..n_y {
            for i in 0..n_x {
                lat.push(Site { x: i as f64, y: j as f64, z: 0.0, occupant:None });
            }
        }
        Square { lat }
    }

    fn get_neighbors(m_x:i32, m_y:i32, n_x:i32, n_y:i32) -> Vec<Site>{
        let mut neighbors:Vec<Site> = Vec::new();
        neighbors.push(Site{x:((m_x + 1 + n_x)%n_x) as f64, y:m_y as f64, z:0.0, occupant:None});
        neighbors.push(Site{x:m_x as f64, y:((m_y + 1 + n_y)%n_y) as f64, z:0.0, occupant:None});
        neighbors
    }
}