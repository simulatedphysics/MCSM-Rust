//pub trait Occupant {}
use std::fmt;

#[derive(Clone)]
pub struct Site {
    x: f64,
    y: f64,
    z: f64,
//    occupant: Option<Occupant>,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

//type Sites = Vec<Site>;

pub trait Lattice {
    fn new(n_x: i32, n_y: i32) -> Self;
//    fn get_neighbors(s: Site) -> Vec<Site>;
}

pub mod square;
//mod triangular;
//mod kagome;
