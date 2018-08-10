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

    pub trait Model {
//        type Lattice: Vec<lattice::Coordinate>;

        fn swap();
        fn energy();
        fn lattice() -> Lattice;
    }
}