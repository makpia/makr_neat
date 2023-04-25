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

    fn is_before<T: Into<TypeLevel>>(
        &self,
        level: T,
    ) -> bool {
        self.level < level.into()
    }
    fn is_after<T: Into<TypeLevel>>(
        &self,
        level: T
    ) -> bool {
        self.level > level.into()
    }
    fn is_same_level<T: Into<TypeLevel>> (
        &self,
        level: T
    ) -> bool {
        self.level == level
    }
}


impl From<NeatPos> for TypeLevel {
    fn from(pos: NeatPos) -> Self {
        pos.level
    }
}
