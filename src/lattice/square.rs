use lattice::{Lattice, Site};
use std::rc::Rc;
use std::rc::Weak;
use std::fmt;
use std::cell::RefCell;

pub struct Square {
    sites: Vec<Rc<Site>>,
    x_width: i32,
    y_width: i32,
}

impl Lattice for Square {
    fn new(x_width: i32, y_width: i32) -> Square {
        let mut sites: Vec<Rc<Site>> = Vec::with_capacity((x_width * y_width) as usize);
        for j in 0..y_width {
            for i in 0..x_width {
                sites.push(
                    Rc::new(Site {
                        x: i as f64,
                        y: j as f64,
                        z: 0.0,
                        occupant: None,
                        neighbors: RefCell::new(Vec::new()),
                    })
                );
            }
        }

        for site in &sites {
            for site_2 in &sites {
                if site.x == ((x_width as f64 + 1.00 + site_2.x) % site_2.x) {
                    if site.y == y_width as f64 {
                        site.neighbors.borrow_mut().push(Rc::downgrade(&site_2))
                    }
                }
            }
        }

        return Square { sites, x_width, y_width };
    }

    fn get_area(self: &Self) -> i32 {
        return self.y_width * self.y_width;
    }

    fn get_sites(self: &Self) -> &Vec<Rc<Site>> {
        return &self.sites;
    }

    fn get_neighbors(self: &Self, site: Site) -> RefCell<Vec<Weak<Site>>> {
        return site.neighbors;
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for site in &self.sites {
            writeln!(f, "({}, {}, {})", site.x, site.y, site.z);
        }
        Ok(())
    }
}