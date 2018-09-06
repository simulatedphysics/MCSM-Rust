use std::fmt;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::ops::Neg;
use std::ops::{Sub, Div};
use plot::CartesianPoint;

pub struct Site {
    x: f64,
    y: f64,
    z: f64,
    occupant: Option<Spin>,
    neighbors: RefCell<Vec<Weak<RefCell<Site>>>>,
}

#[derive(Clone)]
pub struct Spin {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Spin {
    pub fn normalize(&mut self) -> &Self {
        let normalization = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        self.x /= normalization;
        self.y /= normalization;
        self.z /= normalization;
        return self;
    }

    pub fn dot(&self, second_spin: &Spin) -> f64 {
        self.x * second_spin.x + self.y * second_spin.y + self.z * second_spin.z
    }
}

impl Div<f64> for Spin {
    type Output = Self;
    fn div(self, denom: f64) -> Self {
        Spin { x: self.x / denom, y: self.y / denom, z: self.z / denom }
    }
}

impl Sub<Spin> for Spin {
    type Output = Self;
    fn sub(self, other: Spin) -> Self {
        Spin { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl fmt::Display for Spin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}


impl Site {
    pub fn get_neighbors(self: &Self) -> &RefCell<Vec<Weak<RefCell<Site>>>> {
        return &self.neighbors;
    }

    pub fn set_spin(self: &mut Self, spin: Spin) {
        self.occupant = Some(spin)
    }

    pub fn get_spin(&self) -> &Option<Spin> {
        return &self.occupant;
    }
}

//impl Neg for Spin {
//    type Output = Self;
//    fn neg(self) -> Self {
//        Spin { value: -self.value }
//    }
//}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub trait Lattice: fmt::Display {
    fn new(n_x: i32, n_y: i32) -> Self where Self: Sized;
    fn get_area(&self) -> i32;
    fn get_sites(&self) -> &Vec<Rc<RefCell<Site>>>;
}

pub mod square;
//mod triangular;
//mod kagome;
