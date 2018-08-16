pub struct Observables {
    pub average_energy: f64,
    pub average_magneitzation: f64,
    pub magnetic_susceptibility: f64,
    pub heat_capacity: f64,
}

pub trait Model {
    fn swap(&self);
    fn energy(&self);
    fn lattice(&self);
}

//mod ising;
//mod heisenberg;