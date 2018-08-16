//pub trait Occupant {}

pub struct Site {
    x: f64,
    y: f64,
    z: f64,
//    occupant: Option<Occupant>,
}


type Sites = Vec<Site>;

pub trait Lattice {
    fn new(self, n_x: i32, n_y: i32) -> Self;
//    fn get_neighbors(s: Site) -> Vec<Site>;
}

mod square;
mod triangular;
mod kagome;
