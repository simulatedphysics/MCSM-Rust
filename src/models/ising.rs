extern crate rand;
use std::fmt;
use std::ops::{Sub, Div, Neg};
use models::Model;
use self::rand::Rng;
use lattice::Lattice;


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

#[derive(Clone)]
struct Ising {
    spin_configuration: Vec<IsingSpin>
}

impl Ising {
    fn new() -> Ising {
        let mut square_lattice = Lattice::generate_square_lattice(n_x, n_y);
        let neighbor_number = 2;
        let mut ising_spin_configuration: Vec<IsingSpin> = Vec::new();

        for _i in 0..n {
            ising_spin_configuration.push(IsingSpin::create_random_ising_spin());
        }
        Ising { spin_configuration: ising_spin_configuration }
    }
}

impl Model for Ising {
    fn swap() {
        unimplemented!();
    }

    fn energy() -> f64 {
        let j_eng = 1.0;
        let mut energy_total = 0.0;

        for site_index in 0..spin_configuration.spin_configuration.len() {
            let site_spin: IsingSpin = spin_configuration.spin_configuration[site_index];
            let list_of_neighbors: Vec<Site> = Lattice::neighbor_list(square_lattice.lattice[site_index].x as i32, square_lattice.lattice[site_index].y as i32, n_x, n_y);

            for i in 0..neighbor_number {
                energy_total += (-j_eng) * site_spin.spin_value * spin_configuration.spin_configuration[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x) as usize].spin_value
            }
        }
        energy_total
    }

    fn lattice() {
        unimplemented!();
    }
}

