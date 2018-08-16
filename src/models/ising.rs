extern crate rand;


use std::fmt;
use std::ops::Neg;
use models::{Model, Observables};
use self::rand::Rng;

use ::lattice::Lattice;


struct IsingSpin {
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

//#[derive(Clone)]
struct Ising {
    spin_configuration: Vec<IsingSpin>
}

impl Ising {
    fn new(n_x: i32, n_y: i32) -> Ising {
        let mut square_lattice = Lattice::generate_square_lattice(n_x, n_y);
        let neighbor_number = 2;
        let mut spin_configuration: Vec<IsingSpin> = Vec::new();

        for _i in 0..n_x * n_y {
            spin_configuration.push(IsingSpin::new());
        }

        Ising { spin_configuration }
    }
}

impl Model for Ising {
    fn swap(&mut self) -> &Self {
        unimplemented!();
    }

    fn get_energy(&self) -> f64 {
        unimplemented!();
    }

    fn get_lattice(&self) -> Lattice {
        unimplemented!();
    }

    fn measure() -> Observables {
        unimplemented!();
    }
}

