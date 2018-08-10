use models;

mod models {
    struct Spin {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Spin {
        fn new() -> Spin {
            let mut rng = rand::thread_rng();
            Spin { x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>() }
        }

        fn normalized_spin(&mut self) -> Spin {
            let normalization = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
            self.x /= normalization;
            self.y /= normalization;
            self.z /= normalization;
            *self
        }

        fn dot(self, second_spin: Spin) -> f64 {
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

    struct Heisenberg {
        spin_configuration: Vec<Spin>
    }

    impl Heisenberg {
        fn new(system_size: i32) {
            let mut spin_configuration_temp: Vec<Spin> = Vec::new();

            for _i in 0..system_size {
                spin_configuration_temp.push(Spin::normalized_spin(&mut Spin::new()));
            }
        }
    }

    impl Model for Heisenberg {
        fn swap() {
            implement!();
        }

        fn energy() {
            implement!();
        }

        fn lattice() {
            implement!();
        }
    }
}