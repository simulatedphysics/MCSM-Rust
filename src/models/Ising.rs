use models;

mod models {
    struct IsingSpin {
        value: bool
    }

    impl IsingSpin {
        pub fn new() -> IsingSpin {
            let mut rng = rand::thread_rng();
            IsingSpin { value: (rng.gen_range(0, 2)) > 1 }
        }
    }

    impl Neg for IsingSpin {
        type Output = Self;
        fn neg(self) -> Self {
            IsingSpin { value: !self.value }
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
