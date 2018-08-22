//pub trait Occupant {}
use std::fmt;
use models::ising::IsingSpin;

#[derive(Clone)]
pub struct Site<'a> {
    x: f64,
    y: f64,
    z: f64,
    occupant: Option<IsingSpin>,
    neighbors: Vec<Option<&'a Site<'a>>>,
}

impl<'a> fmt::Display for Site<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

//type Sites = Vec<Site>;

pub trait Lattice {
    fn new(n_x: i32, n_y: i32) -> Self where Self: Sized;
    fn get_neighbors(self: &Self, s: Site) -> Vec<&Site>;
}

pub mod square;
//mod triangular;
//mod kagome;
