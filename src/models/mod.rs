use lattice::LatticeObject;

pub struct Observables {
    pub average_energy: f64,
    pub average_magneitzation: f64,
    pub magnetic_susceptibility: f64,
    pub heat_capacity: f64,
}

pub trait Model {
    fn new(l: LatticeObject) -> Self;
    fn flip_spin(&mut self) -> &Self;
    fn get_energy(&self) -> f64;
    fn measure() -> Observables;
}

pub mod ising;
mod heisenberg;