mod models;
mod lattice;


use lattice::{Lattice, Site, square};
use models::ising::{IsingSpin};
//
fn main() {
    let lat: square::Square = Lattice::new(4, 4);

    for i in 0..16 {
        let site: Site = lat.lat[i].clone();
        println!("The site of the lattice is: {}", site);
    }

//    let new_obs = models::Observables
    println!("Ising spin is: {}", IsingSpin::new())

//    let hb = models::

}
