mod models {

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