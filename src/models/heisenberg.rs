extern crate rand;
extern crate num_complex;
extern crate rulinalg;

use std::fmt;
use std::ops::{Sub, Div};
use self::rand::Rng;
use lattice::Site;
use std::cell::RefCell;
use lattice::{Lattice, Spin};
use std::f64;
use plot::CartesianPoint;
use models::{Model, Observables, StateChange};
use std::rc::Rc;
use self::num_complex::Complex;
use self::rulinalg::matrix::{Matrix, BaseMatrix};

#[derive(Clone)]
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
    pub fn phase_factor(pt: Spin, pt2: Spin) -> Complex<f64> {
        let phase: Complex<f64> = self::f64::consts::PI * Complex::i() * pt.dot(&pt2);
        phase.exp()
    }
}

#[derive(Clone)]
pub struct Heisenberg<'a, L: 'a> {
    lattice: &'a L,
    exchange_matrix: ExchangeMatrix,
}


impl<'a, L: Lattice> Model<'a, L> for Heisenberg<'a, L> {
    fn new_spin() -> Spin {
        let mut rng = rand::thread_rng();
        return Spin { x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>() };
    }

    /*

    -- let i1 = init ising
    for n... 10000 {
        -- let i2 = i1.clone()

        -- let change_in_energy = i2.flip_spin()


        if change_in_energy < 0:
            i1 = i2
        }


    */

    fn flip_spin(&mut self) -> StateChange<Self> {
        let mut rng = rand::thread_rng();

        let mut sites = self.lattice.get_sites();

        let random_site_index = rng.gen_range(0, sites.len() - 1) as usize;


        let mut h = Self::new_spin();
        h.normalize();

        return StateChange {
            model: self,
            new_spin: h,
            index: random_site_index,
        };
    }

    fn get_energy(&self) -> f64 {
        let mut energy: f64 = 0.0;

        for (i, site1) in self.lattice.get_sites().iter().enumerate() {
            for (j, site2) in self.lattice.get_sites().iter().enumerate() {
                energy +=
                    site1.borrow().get_spin().as_ref().unwrap().dot(
                        site2.borrow().get_spin().as_ref().unwrap()
                    ) * self.exchange_matrix.select_mat(i, j)
            }
        }

        return energy;
    }

    fn new(l: &'a L) -> Self {
        let mut spin_configuration: Vec<Spin> = Vec::new();
        let x = l.get_sites();

        for site in x {
            let mut h = Self::new_spin();
            h.normalize();
            site.borrow_mut().set_spin(h)
        }

        let exchange_matrix = ExchangeMatrix::ferromagnetic_exchange(l.get_area() as usize);

        return Heisenberg { lattice: l, exchange_matrix };
    }

    fn change_in_energy(index: usize, new_spin: Spin) -> f64 {
        return 0.0;
    }

    fn measure(&self) -> Observables {
        unimplemented!();
    }
}