use crate::neat_pos::neat_pos;

#[allow(non_camel_case_types)]
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
}
