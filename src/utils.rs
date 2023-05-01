use rand::distributions::{
    Distribution,
    Uniform,
};


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
    Uniform::new(-1.0f64, 1.0).sample(&mut rand::thread_rng()) * std + mean
}


pub fn apply_weight(
    values: &[f64],
    weights: &[f64],
) -> f64 {
    if values.len() > 2 {
        assert_eq!(values.len(), weights.len());
        let mut result = 0.0;
        for i in 0..values.len() {
            result += values[i] * weights[i];
        }
        result / weights.sum()
    } else {
        assert_eq!(values.len(), 2);
        assert_eq!(weights.len(), 1);
        values[0] * weights[0] + values[1] * (1.0 - weights[0])
    }
}
