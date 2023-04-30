use rand::distributions::{Distribution, Uniform};


pub fn range_limit<T: PartialOrd>(
    value: T,
    min: T,
    max: T,
) -> T {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

pub fn normalvariate(
    mean: f64,
    std: f64,
) -> f64 {
    Uniform::new(-1.0f64, 1.0)
        .sample(&mut rand::thread_rng())
        * std
        + mean
}
