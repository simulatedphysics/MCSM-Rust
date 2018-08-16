mod lattice;
extern crate sp;
use sp::lattice::{square::Square, Lattice};

fn main() {
    let lat : Square = Lattice::new(4,4);

    println!("Hello, world!");
}
