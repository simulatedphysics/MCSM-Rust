pub struct Observables {
    pub average_energy: f64,
    pub average_magneitzation: f64,
    pub magnetic_susceptibility: f64,
    pub heat_capacity: f64,
}

pub trait Model {
    fn swap(&mut self) -> &Self;
    fn get_energy(&self) -> f64;
    fn get_lattice(&self) -> Lattice;
    fn measure() -> Observables;
}

//mod ising;
//mod heisenberg;