use lattice::{Lattice, Site};
use std::rc::Rc;
use std::rc::Weak;
use std::fmt;
use std::cell::RefCell;

pub struct Square {
    sites: Vec<Rc<RefCell<Site>>>,
    x_width: i32,
    y_width: i32,
}

impl Lattice for Square {
    fn new(x_width: i32, y_width: i32) -> Square {
        let mut sites: Vec<Rc<RefCell<Site>>> = Vec::with_capacity((x_width * y_width) as usize);
        for j in 0..y_width {
            for i in 0..x_width {
                sites.push(
                    Rc::new(RefCell::new(Site {
                        x: i as f64,
                        y: j as f64,
                        z: 0.0,
                        occupant: None,
                        neighbors: RefCell::new(Vec::new()),
                    }))
                );
            }
        }

        for site in &sites {
            let x = site.borrow();
            for site_2 in &sites {
                if x.x == ((x_width as f64 + 1.00 + site_2.borrow().x) % site_2.borrow().x) {
                    if x.y == y_width as f64 {
                        x.neighbors.borrow_mut().push(Rc::downgrade(&site_2))
                    }
                }
            }
        }

        return Square { sites, x_width, y_width };
    }

    fn get_area(self: &Self) -> i32 {
        return self.y_width * self.y_width;
    }

    fn get_sites(self: &Self) -> &Vec<Rc<RefCell<Site>>> {
        return &self.sites;
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for site in &self.sites {
            writeln!(f, "({}, {}, {})", site.borrow().x, site.borrow().y, site.borrow().z);
        }
        Ok(())
    }
}