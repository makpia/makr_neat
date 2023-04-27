extern crate makr_neat;


#[cfg(test)]
mod test_is_output {
    use makr_neat::neat_pos::{NeatPos, TypeLevel};

    #[test]
    fn level_0() {
        assert!(!NeatPos { level: 0, offset: 0 }.is_output());
    }

    #[test]
    fn level_1() {
        assert!(!NeatPos { level: 1, offset: 0 }.is_output())
    }

    #[test]
    fn level_max() {
        assert!(
            NeatPos {
                level: TypeLevel::MAX,
                offset: 0,
            }.is_output())
    }
}



