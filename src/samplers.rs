mod samplers {
    use models;
    use samplers;

    trait Sampler {
        fn run<M, S>(pg: ParameterGroup<M, S>);
    }

    struct ParameterGroup<M, S> {
        model: M,
        sampler: S,
        n_x: Option<i8>,
        n_y: Option<i8>,
        n_z: Option<i8>,
    }

    struct Metropolis {}
}