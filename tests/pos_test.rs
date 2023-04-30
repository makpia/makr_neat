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


#[cfg(test)]
mod test_is_input {
    use makr_neat::neat_pos::{
        neat_pos,
        type_level,
    };

    #[test]
    fn level_0() {
        assert!(neat_pos {
            level: 0,
            offset: 0
        }
        .is_input());
    }

    #[test]
    fn level_1() {
        assert!(!neat_pos {
            level: 1,
            offset: 0
        }
        .is_input())
    }

    #[test]
    fn level_max() {
        assert!(!neat_pos {
            level: type_level::MAX,
            offset: 0,
        }
        .is_input())
    }
}


#[cfg(test)]
mod test_is_hidden {
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
        .is_hidden());
    }

    #[test]
    fn level_1() {
        assert!(neat_pos {
            level: 1,
            offset: 0
        }
        .is_hidden())
    }

    #[test]
    fn level_max() {
        assert!(!neat_pos {
            level: type_level::MAX,
            offset: 0,
        }
        .is_hidden())
    }
}


#[cfg(test)]
mod test_output_pos {
    use makr_neat::neat_pos::neat_pos;

    #[test]
    fn is_not_input() {
        assert!(!neat_pos::output_pos(0).is_input())
    }

    #[test]
    fn is_output() {
        assert!(neat_pos::output_pos(0).is_output())
    }

    #[test]
    fn is_not_hidden() {
        assert!(!neat_pos::output_pos(0).is_hidden())
    }
}


#[cfg(test)]
mod test_input_pos {
    use makr_neat::neat_pos::neat_pos;

    #[test]
    fn is_input() {
        assert!(neat_pos::input_pos(0).is_input())
    }

    #[test]
    fn is_not_output() {
        assert!(!neat_pos::input_pos(0).is_output())
    }

    #[test]
    fn is_not_hidden() {
        assert!(!neat_pos::input_pos(0).is_hidden())
    }
}


#[cfg(test)]
mod test_is_before {
    use makr_neat::neat_pos::neat_pos;

    #[test]
    fn lower() {
        assert!(neat_pos {
            level: 0,
            offset: 0
        }
        .is_before(1u32))
    }

    #[test]
    fn same() {
        assert!(!neat_pos {
            level: 0,
            offset: 0
        }
        .is_before(0u32))
    }

    #[test]
    fn higher() {
        assert!(!neat_pos {
            level: 1,
            offset: 0
        }
        .is_before(0u32))
    }
}


#[cfg(test)]
mod test_is_after {
    use makr_neat::neat_pos::neat_pos;

    #[test]
    fn lower() {
        assert!(!neat_pos {
            level: 0,
            offset: 0
        }
        .is_after(1u32))
    }

    #[test]
    fn same() {
        assert!(!neat_pos {
            level: 0,
            offset: 0
        }
        .is_after(0u32))
    }

    #[test]
    fn higher() {
        assert!(neat_pos {
            level: 1,
            offset: 0
        }
        .is_after(0u32))
    }
}


#[cfg(test)]
mod test_is_same_level {
    use makr_neat::neat_pos::neat_pos;

    #[test]
    fn lower() {
        assert!(!neat_pos {
            level: 0,
            offset: 0
        }
        .is_same_level(1u32))
    }

    #[test]
    fn same() {
        assert!(neat_pos {
            level: 0,
            offset: 0
        }
        .is_same_level(0u32))
    }

    #[test]
    fn higher() {
        assert!(!neat_pos {
            level: 1,
            offset: 0
        }
        .is_same_level(0u32))
    }
}


#[cfg(test)]
mod test_cast {
    use makr_neat::neat_pos::neat_pos;

    #[test]
    fn cast() {
        let casted: u32 = neat_pos {
            level: 0,
            offset: 0,
        }
        .into();
        assert_eq!(casted, 0u32)
    }
}
