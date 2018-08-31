use lattice::Lattice;
use plot::CartesianPoint;

pub struct Observables {
    pub average_energy: f64,
    pub average_magneitzation: f64,
    pub magnetic_susceptibility: f64,
    pub heat_capacity: f64,
    pub spin_states: Vec<CartesianPoint>,
}

pub trait Model<'a> {
    fn new<L: Lattice>(l: &'a L) -> Self where Self: Sized;
    fn flip_spin(&mut self) -> &Self;
    fn get_energy(&self) -> f64;
    fn measure(&self) -> Observables;
}

pub mod ising;
pub mod heisenberg;