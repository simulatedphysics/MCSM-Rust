mod lattice {

    pub trait Occupant {

    }

    pub struct Site {
        x: f64,
        y: f64,
        z: f64,
        occupant: Option<Occupant>,
    }


        type Sites = Vec<Site>;
    trait Lattice: Debug {
        fn get_neighbors(s: Site) -> Vec<Site>;
        fn new(n_x_size: usize, n_y_size: usize);
    }
}