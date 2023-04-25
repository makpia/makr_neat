// todo: find the fastest type for this
type TypeLevel = u32;
type TypeOffset = u32;


#[derive(Copy, Clone, Debug)]
struct NeatPos {
    level: TypeLevel,
    offset: TypeOffset,
}


impl NeatPos {
    fn is_output(&self) -> bool {
        self.level == TypeLevel::MAX
    }
    fn is_input(&self) -> bool {
        self.level == 0
    }
    fn is_hidden(&self) -> bool {
        0 < self.level && self.level < TypeLevel::MAX
    }

    fn output_pos(offset: u32) -> NeatPos {
        NeatPos {
            level: TypeLevel::MAX,
            offset,
        }
    }
    fn input_pos(offset: u32) -> NeatPos {
        NeatPos {
            level: 0,
            offset,
        }
    }

    fn is_before(&self, other: &NeatPos) -> bool {
        self.level < other.level
    }
    fn is_after(&self, other: &NeatPos) -> bool {
        self.level > other.level
    }
    fn is_same_level(&self, other: &NeatPos) -> bool {
        self.level == other.level
    }
}
