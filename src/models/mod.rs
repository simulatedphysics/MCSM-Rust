mod models {
    mod lattice {

        pub trait Occupant {

        }

        pub struct Site {
            x: f64,
            y: f64,
            z: f64,
            occupant: Option<Occupant>,
    }


        trait Lattice {
            type Sites = Vec<Site>;
            fn get_neighbors(s: Site) -> Vec<Site>;
        }

        mod structures {
            struct Kagome {}

            struct Square {}

            struct Triangular {}
        }
    }

    pub struct Observables{
        pub average_energy: f64,
        pub average_magneitzation: f64,
        pub magnetic_susceptibility: f64,
        pub heat_capacity: f64,
    }

    pub trait Model {
//        type Lattice: Vec<lattice::Coordinate>;

        fn swap();
        fn energy();
        fn lattice() -> Lattice;
    }
}