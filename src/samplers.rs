mod sampler {
    use models;
    use samplers;

    trait Sampler {
        fn run<M, S>(pg: ParameterGroup<M, S>);
    }

    struct ParameterGroup<M, S> {
        model: M,
        sampler: S,
    }

    struct Metropolis {}
}