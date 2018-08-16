extern crate sp;
use sp::lattice::{square::Square, Lattice};

fn main() {
    let lat : Square = Lattice::new(4,4);

    for i in 0..16{
        let site: sp::lattice::Site = lat.lat[i].clone();
        println!("The site of the lattice is: {}", site);
    }
}
