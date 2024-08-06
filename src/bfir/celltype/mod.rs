use core::ops::{
    Add,
    Sub,
};

use num_traits::identities::{
    One,
    Zero
};

use num_traits::ops::{
    overflowing::{
        OverflowingAdd,
        OverflowingSub
    },

    saturating::{
        SaturatingAdd,
        SaturatingSub
    },

    wrapping::{
        WrappingAdd,
        WrappingSub
    }
};


pub trait BFCell:
    Copy +
    Eq +
    Zero +
    One +
    Add<Self, Output = Self> +
    Sub<Self, Output = Self> +
    OverflowingAdd +
    OverflowingSub +
    SaturatingAdd +
    SaturatingSub +
    WrappingAdd +
    WrappingSub {}


impl BFCell for u8   {}
impl BFCell for u16  {}
impl BFCell for u32  {}
impl BFCell for u64  {}
impl BFCell for u128 {}

