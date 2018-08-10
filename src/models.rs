mod models {
    mod lattice {
        pub struct Coordinate {
            x: f64,
            y: f64,
            z: f64,
        }
    }

    pub trait Model {
//        type Lattice: Vec<lattice::Coordinate>;

        fn swap();
        fn energy();
        fn lattice() -> Lattice;
    }

}