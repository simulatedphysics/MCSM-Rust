use models;

mod models {
    struct IsingSpin {
        value: bool
    }

    impl IsingSpin {
        pub fn create_random_ising_spin() -> IsingSpin {
            let mut rng = rand::thread_rng();
            IsingSpin { value: (rng.gen_range(0, 2) * 2 - 1) as bool }
        }
    }

    impl Sub<IsingSpin> for IsingSpin {
        type Output = Self;
        fn sub(self, other: IsingSpin) -> Self {
            IsingSpin { value: self.value - other.value }
        }
    }

    impl Mul<IsingSpin> for IsingSpin {
        type Output = Self;
        fn mul(self, other: IsingSpin) -> Self {
            IsingSpin { value: self.value * other.value }
        }
    }

    impl Mul<f64> for IsingSpin {
        type Output = Self;
        fn mul(self, constant: f64) -> Self {
            IsingSpin { value: self.value * constant }
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
            write!(f, "({})", self.spin_value)
        }
    }


    struct Ising {
        spin_configuration: Vec<IsingSpin>
    }

    impl Ising {
        fn new() -> Ising {
            let mut ising_spin_configuration: Vec<IsingSpin> = Vec::new();

            for _i in 0..n {
                ising_spin_configuration.push(IsingSpin::create_random_ising_spin());
            }
            Ising { spin_configuration: ising_spin_configuration }
        }
    }
}
