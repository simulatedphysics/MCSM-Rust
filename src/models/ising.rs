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
pub struct Ising<'a> {
    spin_configuration: Vec<IsingSpin>,
    lattice: &'a Lattice,
}

impl<'a> Model<'a> for Ising<'a> {
    fn new<L: Lattice>(lattice: &'a L) -> Self {
        let mut spin_configuration: Vec<IsingSpin> = Vec::new();

        for _i in 0..lattice.get_area() {
            spin_configuration.push(IsingSpin::new());
        }

        let mut i: Ising = Ising { spin_configuration, lattice };

        return i;
    }

    fn flip_spin(&mut self) -> &Self {
        unimplemented!();
    }

    fn get_energy(&self) -> f64 {
        unimplemented!();
    }

    fn measure(&self) -> Observables {
        unimplemented!();
    }
}

