use proptest::prelude::*;

prop_compose! {
    pub fn positive_byte()(v in 0b0000_0001..0b0111_1111u8) -> u8 {
        v
    }
}

prop_compose! {
    pub fn negative_byte()(v in 0b1000_0000..0b1111_1111u8) -> u8 {
        v
    }
}
