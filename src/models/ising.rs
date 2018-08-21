extern crate rand;

use std::fmt;
use std::ops::Neg;
use models::{Model, Observables};
use self::rand::Rng;
use lattice::Lattice;


#[derive(Clone)]
pub struct IsingSpin {
    value: f64
}

impl IsingSpin {
    pub fn new() -> IsingSpin {
        let mut rng = rand::thread_rng();
        IsingSpin { value: (2 * rng.gen_range(0, 2)) as f64 - 1.0 }
    }
}

impl Neg for IsingSpin {
    type Output = Self;
    fn neg(self) -> Self {
        IsingSpin { value: -self.value }
    }
}

impl fmt::Display for IsingSpin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.value)
    }
}

#[derive(Clone)]
struct Ising {
    spin_configuration: Vec<IsingSpin>
}

impl Model for Ising {
    fn new<L: Lattice>(l: L) -> Self {
        let mut ising_spin_configuration: Vec<IsingSpin> = Vec::new();

//        for _i in 0 .. len(l) {
        for _i in 0..10 {
            ising_spin_configuration.push(IsingSpin::new());
        }

        let mut i: Ising = Ising { spin_configuration: ising_spin_configuration };

        return i;
    }

    fn flip_spin(&mut self) -> &Self {
        unimplemented!();
    }

    fn get_energy(&self) -> f64 {
        unimplemented!();
    }

    fn measure() -> Observables {
        unimplemented!();
    }
}

