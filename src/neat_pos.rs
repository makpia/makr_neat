// todo: find the fastest type for this
pub type type_level = u32;
pub type type_offset = u32;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct neat_pos {
    pub level: type_level,
    pub offset: type_offset,
}


impl neat_pos {
    pub fn is_output(&self) -> bool {
        self.level == type_level::MAX
    }
    pub fn is_input(&self) -> bool {
        self.level == 0
    }
    pub fn is_hidden(&self) -> bool {
        0 < self.level && self.level < type_level::MAX
    }

    pub fn output_pos(offset: u32) -> neat_pos {
        neat_pos {
            level: type_level::MAX,
            offset,
        }
    }
    pub fn input_pos(offset: u32) -> neat_pos {
        neat_pos {
            level: 0,
            offset,
        }
    }

    pub fn is_before<T: Into<type_level>>(
        &self,
        level: T,
    ) -> bool {
        self.level < level.into()
    }
    pub fn is_after<T: Into<type_level>>(
        &self,
        level: T,
    ) -> bool {
        self.level > level.into()
    }
    pub fn is_same_level<T: Into<type_level>>(
        &self,
        level: T,
    ) -> bool {
        self.level == level.into()
    }
}


impl From<neat_pos> for type_level {
    fn from(pos: neat_pos) -> Self {
        pos.level
    }
}
