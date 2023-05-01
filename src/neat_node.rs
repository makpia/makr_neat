use std::collections::HashMap;

use crate::neat_activation_func::neat_activation_func_types;
use crate::neat_connection::neat_connection;
use crate::neat_pos::neat_pos;

pub struct neat_node {
    pub pos: neat_pos,

    pub activation_func: neat_activation_func_types,
    pub bias: f64,

    pub connections: HashMap<neat_pos, neat_connection>,
}


impl neat_node {
    pub fn check_valid(&self) {
        // - make sure the hash keys are the same as the pos_from of connections
        for (&pos, &connection) in &self.connections {
            assert_eq!(connection.pos_from, pos);
        }
    }
}
