mod models;
mod lattice;

use models::Model;

use lattice::{Lattice, square};


fn main() {
    let lat: square::Square = Lattice::new(4, 4);

    println!("{}", lat);

    let mut model: models::ising::Ising = models::Model::new(lat);

    for _ in 0..10 {
        model.flip_spin();
        println!("{}", model.get_energy());
    }
}
