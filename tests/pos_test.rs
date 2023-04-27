extern crate makr_neat;


#[cfg(test)]
mod test_is_output {
    use makr_neat::neat_pos::{
        neat_pos,
        type_level,
    };

    #[test]
    fn level_0() {
        assert!(!neat_pos {
            level: 0,
            offset: 0
        }
        .is_output());
    }

    #[test]
    fn level_1() {
        assert!(!neat_pos {
            level: 1,
            offset: 0
        }
        .is_output())
    }

    #[test]
    fn level_max() {
        assert!(neat_pos {
            level: type_level::MAX,
            offset: 0,
        }
        .is_output())
    }
}
