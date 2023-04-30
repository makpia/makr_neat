use crate::neat_config::neat_chromosome_config;
use crate::neat_pos::neat_pos;
use crate::utils::range_limit;

#[derive(Copy, Clone, Debug)]
struct neat_connection {
    pub pos_from: neat_pos,
    pub weight: f64,
}

impl neat_connection {
    pub fn diff(
        conn1: neat_connection,
        conn2: neat_connection,
    ) -> f64 {
        (conn1.weight - conn2.weight).abs()
    }

    pub fn diff_squared(
        conn1: neat_connection,
        conn2: neat_connection,
    ) -> f64 {
        (conn1.weight - conn2.weight).powi(2)
    }

    pub fn weight_range_limit(
        &mut self,
        config: neat_chromosome_config,
    ) {
        self.weight = range_limit(
            self.weight,
            config.connection_limit_weight_min,
            config.connection_limit_weight_max,
        );
    }
    pub fn initialize_random(
        pos_from: neat_pos,
        config: neat_chromosome_config,
    ) -> neat_connection {
        neat_connection {
            pos_from,
            weight: config.connection_weight_random(),
        }
    }
}
