use lattice::{Lattice, Site};

pub struct Square<'a> {
    sites: Vec<Site<'a>>,
    x_width: i32,
    y_width: i32,
}

//impl Square {
//    pub fn map_to_index(m_x: i32, m_y: i32, n_x: i32) -> i32 {
//        m_y * n_x + m_x
//    }
//}
//
//impl Square {
//    pub fn map_to_site(index: i32, n_x: i32) -> Site {
//        Site { x: (index - n_x * (index / n_x)) as f64, y: (index / n_x) as f64, z: 0.0, occupant: None, neighbors: None }
//    }
//}

impl<'a> Lattice for Square<'a> {
    fn new(x_width: i32, y_width: i32) -> Square<'a> {
        let mut sites: Vec<Site<'a>> = Vec::with_capacity((x_width * y_width) as usize);
        for j in 0..y_width {
            for i in 0..x_width {
                sites.push(
                    Site { x: i as f64, y: j as f64, z: 0.0, occupant: None, neighbors: Vec::new() }
                );
            }
        }

        for site in &mut sites {
            for site_2 in &sites {
                if site.x == ((x_width as f64 + 1.00 + site_2.x) % site_2.x) {
                    if site.y == y_width as f64 {
                        site.neighbors.push(Some(site_2))
                    }
                }
            }
        }

        Square { sites, x_width, y_width }
    }

    fn get_neighbors(self: &Self, site: Site) -> Vec<&Site> {
        unimplemented!()
//        let mut neighbors: Vec<Site> = Vec::new();

//        for n in self.sites {
//            if n.x == ((site.x + 1 + self.x_width) % self.x_width) &&
//                n.y == site.y
//                && n.z == 0 {
//                neighbors.push(n)
//            }
//            if n.x == site.x &&
//                n.y == ((self.y_width + 1 + site.y) % site.y)
//                && n.z == 0 {
//                neighbors.push(n)
//            }
//        }


//        neighbors.push(Site { x: ((m_x + 1 + n_x) % n_x) as f64, y: m_y as f64, z: 0.0, occupant: None });
//        neighbors.push(Site { x: m_x as f64, y: ((m_y + 1 + n_y) % n_y) as f64, z: 0.0, occupant: None });
//        neighbors
    }
}