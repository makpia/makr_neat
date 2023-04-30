#![allow(non_camel_case_types)]


pub struct neat_chromosome_config {
    connection_limit_weight_max: f64,
    connection_limit_weight_min: f64,

    connection_initialize_weight_mean: f64,
    connection_initialize_weight_std: f64,
}


impl Default for neat_chromosome_config {
    fn default() -> Self {
        neat_chromosome_config {
            connection_limit_weight_max: 10.0,
            connection_limit_weight_min: -10.0,

            connection_initialize_weight_mean: 0.0,
            connection_initialize_weight_std: 1.0,
        }
    }
}
