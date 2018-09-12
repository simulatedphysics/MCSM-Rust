extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


mod models;
mod lattice;
mod plot;

use models::Model;
use models::heisenberg::Heisenberg;
use models::ising::Ising;

use lattice::{Lattice, square};


fn main() {
    hb();

//    ising();
}

fn hb() {
    for _ in 0..200000 {
        let lat2: square::Square = square::Square::new(4, 4);
        let mut hb = models::heisenberg::Heisenberg::new(&lat2);

        let change_in_energy = hb.flip_spin();
        if change_in_energy.calculate_change_in_energy() < 0.0 {
//            hb = hb;
        }
    }

//    println!("{}", hb.get_energy());
}

fn ising() {
    let lat: square::Square = Lattice::new(4, 4);

    println!("{}", lat);

    let mut model: Ising<square::Square> = models::Model::new(&lat);
}
