extern crate rand;

use std::fmt;
use std::ops::{Sub, Div};
use self::rand::Rng;
use models::Model;

pub struct HeisenbergSpin {
    x: f64,
    y: f64,
    z: f64,
}

impl HeisenbergSpin {
    fn new() -> HeisenbergSpin {
        let mut rng = rand::thread_rng();
        HeisenbergSpin { x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>() }
    }

    fn normalized_spin(&mut self) -> HeisenbergSpin {
        let normalization = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        self.x /= normalization;
        self.y /= normalization;
        self.z /= normalization;
        *self
    }

    fn dot(self, second_spin: HeisenbergSpin) -> f64 {
        self.x * second_spin.x + self.y * second_spin.y + self.z * second_spin.z
    }
}

impl Div<f64> for HeisenbergSpin {
    type Output = Self;
    fn div(self, denom: f64) -> Self {
        HeisenbergSpin { x: self.x / denom, y: self.y / denom, z: self.z / denom }
    }
}

impl Sub<HeisenbergSpin> for HeisenbergSpin {
    type Output = Self;
    fn sub(self, other: HeisenbergSpin) -> Self {
        HeisenbergSpin { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl fmt::Display for HeisenbergSpin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

struct Heisenberg {
    spin_configuration: Vec<HeisenbergSpin>,
    system_size: i32,
}

impl Heisenberg {
    fn new(system_size: i32) -> Heisenberg {
        let mut spin_configuration: Vec<HeisenbergSpin> = Vec::new();
        for _i in 0..system_size {
            spin_configuration.push(HeisenbergSpin::normalized_spin(&mut HeisenbergSpin::new()));
        }

        return Heisenberg { spin_configuration, system_size };
    }
}

impl Model for Heisenberg {
    fn swap(&self) {
        unimplemented!();
    }

    fn energy(&self) {
        unimplemented!();
    }

    fn lattice(&self) {
        unimplemented!();
    }
}