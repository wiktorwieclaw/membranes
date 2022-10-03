use std::num::Wrapping;

pub trait WrappingU16Ext {
    fn cast_wrapping_u8(self) -> Wrapping<u8>;
}

impl WrappingU16Ext for Wrapping<u16> {
    fn cast_wrapping_u8(self) -> Wrapping<u8> {
        Wrapping(self.0 as u8)
    }
}

pub trait WrappingU8Ext {
    // I named it `into` instead of `cast` because the conversion is loseless
    fn into_wrapping_u16(self) -> Wrapping<u16>;
}

impl WrappingU8Ext for Wrapping<u8> {
    fn into_wrapping_u16(self) -> Wrapping<u16> {
        Wrapping(self.0.into())
    }
}
