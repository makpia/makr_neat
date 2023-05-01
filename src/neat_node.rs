use std::collections::{
    HashMap,
    HashSet,
};

use crate::neat_activation_func::neat_activation_func_types;
use crate::neat_config::neat_chromosome_config;
use crate::neat_connection::neat_connection;
use crate::neat_pos::neat_pos;
use crate::utils::apply_weight;

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

    fn _connections_diff(
        node1: &neat_node,
        node2: &neat_node,
        config: neat_chromosome_config,
    ) -> f64 {
        let node1_pos_set: HashSet<neat_pos> = node1.connections.keys().into();
        let node2_pos_set: HashSet<neat_pos> = node2.connections.keys().into();
        let same_pos_set: HashSet<neat_pos> =
            node1_pos_set.intersection(&node2_pos_set).into();

        let mut same_pos_diff = 0.0;
        for pos in same_pos_set {
            let conn1 = node1.connections[&pos];
            let conn2 = node2.connections[&pos];
            same_pos_diff += neat_connection::diff(conn1, conn2);
        }
        same_pos_diff /= same_pos_set.len() as f64;

        let all_pos_set: HashSet<neat_pos> =
            node1_pos_set.union(&node2_pos_set).into();
        let same_pos_ratio: f64 =
            same_pos_set.len() as f64 / all_pos_set.len() as f64;

        apply_weight(
            &[same_pos_diff, config.node_diff_different_pos_from],
            &[same_pos_ratio],
        )
    }

    pub fn diff(
        node1: &neat_node,
        node2: &neat_node,
        config: neat_chromosome_config,
    ) -> f64 {
        let af_diff = if node1.activation_func == node2.activation_func {
            0.0
        } else {
            config.node_diff_different_af.clone()
        };

        let bias_diff = (node1.bias - node2.bias).abs();

        let connections_diff =
            neat_node::_connections_diff(node1, node2, config);

        apply_weight(
            &[af_diff, bias_diff, connections_diff],
            &[
                config.node_diff_weight_af.clone(),
                config.node_diff_weight_bias.clone(),
                config.node_diff_weight_connections.clone(),
            ],
        )
    }
}
