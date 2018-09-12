use lattice::Lattice;
use plot::CartesianPoint;
use std::cell::RefCell;
use lattice::Spin;
use std::rc::Rc;

pub struct Observables {
    pub average_energy: f64,
    pub average_magneitzation: f64,
    pub magnetic_susceptibility: f64,
    pub heat_capacity: f64,
    pub spin_states: Vec<CartesianPoint>,
}

pub struct StateChange<'a, T: 'a> {
    pub model: &'a T,
    pub new_spin: Spin,
    pub index: usize,
}

impl<'a, T> StateChange<'a, T> {
    pub fn calculate_change_in_energy(&self) -> f64 {
        return 0.0;
    }

    pub fn accept(&self) {}

    pub fn reject(&self) {}
}

pub trait Model<'a, L>: Sized {
    fn new(l: &'a L) -> Self where Self: Sized;
    fn flip_spin(&mut self) -> StateChange<Self>;
    fn get_energy(&self) -> f64;
    fn new_spin() -> Spin;
    fn change_in_energy(index: usize, new_spin: Spin) -> f64;
    fn measure(&self) -> Observables;
}

pub mod ising;
pub mod heisenberg;