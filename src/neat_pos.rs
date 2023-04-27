// todo: find the fastest type for this
pub type TypeLevel = u32;
pub type TypeOffset = u32;


#[derive(Copy, Clone, Debug)]
pub struct NeatPos {
    pub level: TypeLevel,
    pub offset: TypeOffset,
}


impl NeatPos {
    pub fn is_output(&self) -> bool {
        self.level == TypeLevel::MAX
    }
    pub fn is_input(&self) -> bool {
        self.level == 0
    }
    pub fn is_hidden(&self) -> bool {
        0 < self.level && self.level < TypeLevel::MAX
    }

    pub fn output_pos(offset: u32) -> NeatPos {
        NeatPos {
            level: TypeLevel::MAX,
            offset,
        }
    }
    pub fn input_pos(offset: u32) -> NeatPos {
        NeatPos {
            level: 0,
            offset,
        }
    }

    pub fn is_before<T: Into<TypeLevel>>(
        &self,
        level: T,
    ) -> bool {
        self.level < level.into()
    }
    pub fn is_after<T: Into<TypeLevel>>(
        &self,
        level: T,
    ) -> bool {
        self.level > level.into()
    }
    pub fn is_same_level<T: Into<TypeLevel>>(
        &self,
        level: T,
    ) -> bool {
        self.level == level.into()
    }
}


impl From<NeatPos> for TypeLevel {
    fn from(pos: NeatPos) -> Self {
        pos.level
    }
}
