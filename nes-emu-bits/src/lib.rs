use std::num::Wrapping;

pub mod prelude {
    pub use super::{Wu16, Wu16Ext, Wu8, Wu8Ext};
}

pub type Wu8 = Wrapping<u8>;
pub type Wu16 = Wrapping<u16>;

pub trait Wu16Ext {
    fn cast_wu8(self) -> Wu8;
}

impl Wu16Ext for Wu16 {
    fn cast_wu8(self) -> Wu8 {
        Wrapping(self.0 as u8)
    }
}

pub trait Wu8Ext {
    // I named it `into` instead of `cast` because the conversion is loseless
    fn into_wu16(self) -> Wu16;
}

impl Wu8Ext for Wu8 {
    fn into_wu16(self) -> Wu16 {
        Wrapping(self.0.into())
    }
}
