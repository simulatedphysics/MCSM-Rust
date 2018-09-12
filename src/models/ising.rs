extern crate rand;

use std::fmt;
use std::ops::Neg;
use models::{Model, Observables, StateChange};
use self::rand::Rng;
use lattice::Spin;
use lattice::Lattice;

#[derive(Clone)]
pub struct Ising<'a, L: 'a> {
    lattice: &'a L,
}

impl<'a, L: Lattice> Model<'a, L> for Ising<'a, L> {
    fn new(lattice: &'a L) -> Self {
//        let mut spin_configuration: Vec<IsingSpin> = Vec::new();

//        for _i in 0..lattice.get_area() {
//            spin_configuration.push(IsingSpin::new());
//        }

        return Ising { lattice };
    }

    fn new_spin() -> Spin {
        let mut rng = rand::thread_rng();
        return Spin { x: 0.0, y: 0.0, z: ((2 * rng.gen_range(0, 2)) - 1) as f64 };
    }

    fn flip_spin(&mut self) -> StateChange<Self> {
        unimplemented!();
    }

    fn get_energy(&self) -> f64 {
        let x = self.lattice.get_sites();
        for site in x {
//            println!("{}", site.get_neighbors())
        }

        return 10.0;
    }

    fn change_in_energy(index: usize, new_spin: Spin) -> f64 {
        return 0.0;
    }

    fn measure(&self) -> Observables {
        unimplemented!();
    }
}

