use std::fmt;
use models::ising::IsingSpin;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use plot::CartesianPoint;

pub struct Site {
    x: f64,
    y: f64,
    z: f64,
    occupant: Option<Box<Spin>>,
    neighbors: RefCell<Vec<Weak<Site>>>,
}

pub trait Spin {
    fn get_cartesian_point(self: &Self) -> CartesianPoint;
}

impl Site {
    pub fn get_neighbors(self: &Self) -> RefCell<Vec<Weak<Site>>> {
        unimplemented!()
//        return self.neighbors;
    }

    pub fn set_occupant(self: &mut Self, spin: Box<Spin>) {
        self.occupant = Some(spin)
    }
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub trait Lattice: fmt::Display {
    fn new(n_x: i32, n_y: i32) -> Self where Self: Sized;
    fn get_area(self: &Self) -> i32;
    fn get_sites(self: &Self) -> &Vec<Rc<Site>>;
}

pub mod square;
//mod triangular;
//mod kagome;
