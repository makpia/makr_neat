#![allow(non_camel_case_types)]

use crate::utils::normalvariate;

pub struct neat_chromosome_config {
    pub connection_limit_weight_max: f64,
    pub connection_limit_weight_min: f64,

    pub connection_initialize_weight_mean: f64,
    pub connection_initialize_weight_std: f64,

    pub node_diff_different_af: f64,
    pub node_diff_different_pos_from: f64,

    pub node_diff_weight_af: f64,
    pub node_diff_weight_bias: f64,
    pub node_diff_weight_connections: f64,
}


impl Default for neat_chromosome_config {
    fn default() -> Self {
        neat_chromosome_config {
            connection_limit_weight_max: 10.0,
            connection_limit_weight_min: -10.0,

            connection_initialize_weight_mean: 0.0,
            connection_initialize_weight_std: 1.0,

            node_diff_different_af: 10.0,
            node_diff_different_pos_from: 5.0,

            node_diff_weight_af: 1.0,
            node_diff_weight_bias: 1.0,
            node_diff_weight_connections: 3.0,
        }
    }
}


impl neat_chromosome_config {
    pub fn connection_weight_random(&self) -> f64 {
        normalvariate(
            self.connection_initialize_weight_mean,
            self.connection_initialize_weight_std,
        )
    }
}
