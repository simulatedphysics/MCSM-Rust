extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


mod models;
mod lattice;
mod plot;
use models::Model;

use lattice::{Lattice, square};


fn main() {

    hb();

    ising();
}

fn hb() {
    let lat2: square::Square = Lattice::new(4, 4);
    let mut hb: models::heisenberg::Heisenberg = models::heisenberg::Heisenberg::new(&lat2);

    for _ in 0..1000 {
        let mut active_hb = hb.clone();

        let pre_flip_energy = active_hb.get_energy();
        active_hb.flip_spin();
        let post_flip_energy = active_hb.get_energy();

        if post_flip_energy < pre_flip_energy {
            hb = active_hb.clone();
        }
    }

    println!("{}", hb.get_energy());
}

fn ising() {
    let lat: square::Square = Lattice::new(4, 4);

    println!("{}", lat);

    let mut model: models::ising::Ising = models::Model::new(&lat);

}
