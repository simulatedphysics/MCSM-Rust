extern crate rand;
extern crate num_complex;
extern crate rulinalg;

use std::fmt;
use std::ops::{Sub, Div};
use self::rand::Rng;
use std::f64;
use models::{Model, Observables};
use self::num_complex::Complex;
use self::rulinalg::matrix::{Matrix, BaseMatrix};

pub struct ExchangeMatrix {
    pub exchange_matrix: Matrix<f64>
}

impl ExchangeMatrix {
    pub fn ferromagnetic_exchange(n: usize) -> ExchangeMatrix {
        ExchangeMatrix { exchange_matrix: -Matrix::new(n, n, (1..(n.pow(2) + 1)).map(|_v| 1 as f64).collect::<Vec<f64>>()) }
    }

    pub fn antiferromagnetic_exchange(n: usize) -> ExchangeMatrix {
        ExchangeMatrix { exchange_matrix: Matrix::new(n, n, (1..(n.pow(2) + 1)).map(|_v| 1 as f64).collect::<Vec<f64>>()) }
    }

    pub fn select_mat(&self, r: usize, c: usize) -> f64 {
        self.exchange_matrix.select(&[r], &[c]).data()[0]
    }
}


impl ExchangeMatrix {
    pub fn phase_factor(pt: HeisenbergSpin, pt2: HeisenbergSpin) -> Complex<f64> {
        let phase: Complex<f64> = self::f64::consts::PI * Complex::i() * pt.dot(pt2);
        phase.exp()
    }
}


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

    fn normalize(&mut self) -> HeisenbergSpin {
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
    exchange_matrix: ExchangeMatrix,
}

impl Heisenberg {
    fn new(system_size: i32) -> Heisenberg {
        let mut spin_configuration: Vec<HeisenbergSpin> = Vec::new();
        for _i in 0..system_size {
            spin_configuration.push(HeisenbergSpin::new().normalize());
        }

        let exchange_matrix = ExchangeMatrix::ferromagnetic_exchange(10);

        return Heisenberg { spin_configuration, system_size, exchange_matrix };
    }
}

impl Model for Heisenberg {
    fn swap(&mut self) -> &Self {
        let mut rng = rand::thread_rng();
        self.spin_configuration[rng.gen_range(0, self.system_size - 1) as usize] =
            HeisenbergSpin::new().normalize();

        return self;
    }

    fn get_energy(&self) -> f64 {
        let mut energy: f64 = 0.0;

        for i in 0..10 {
            for j in 0..10 {
                energy += (self.spin_configuration[i]).dot(self.spin_configuration[j]) * self.exchange_matrix.select_mat(i, j)
            }
        }

        return energy;
    }

    fn get_lattice(&self) -> Lattice {
        unimplemented!();
    }

    fn measure() -> Observables {
        unimplemented!();
    }
}