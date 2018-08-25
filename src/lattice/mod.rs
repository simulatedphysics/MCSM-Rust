use std::fmt;
use models::ising::IsingSpin;
use std::rc::Weak;
use std::cell::RefCell;


#[derive(Clone)]
pub struct Site {
    x: f64,
    y: f64,
    z: f64,
    occupant: Option<IsingSpin>,
    neighbors: RefCell<Vec<Weak<Site>>>,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub trait Lattice: fmt::Display {
    fn new(n_x: i32, n_y: i32) -> Self where Self: Sized;
    fn get_area(self: &Self) -> i32;
    fn get_neighbors(self: &Self, s: Site) -> RefCell<Vec<Weak<Site>>>;
}

pub mod square;
//mod triangular;
//mod kagome;
