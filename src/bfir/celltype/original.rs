use super::*;


pub type Cell8 = u8;
pub type Cell16 = u16;
pub type Cell32 = u32;
pub type Cell64 = u64;
pub type Cell128 = u128;


macro_rules! auto_impl {
    ($method:ident, $t:ty) => {
        #[inline]
        fn $method(self, v: Self) -> Self {
            <$t>::$method(self, v)
        }
    };

    ($method:ident, $t:ty, $ret:ty) => {
        #[inline]
        fn $method(self, v: Self) -> $ret {
            <$t>::$method(self, v)
        }
    };
}


macro_rules! impl_bfcell {
    ($t:ty, $size:expr) => {
        impl BFCell for $t {
            auto_impl!(overflowing_add, $t, (Self, bool));
            auto_impl!(overflowing_sub, $t, (Self, bool));
            auto_impl!(saturating_add, $t);
            auto_impl!(saturating_sub, $t);
            auto_impl!(wrapping_add, $t);
            auto_impl!(wrapping_sub, $t);

            #[inline]
            fn zero() -> Self { 0 }
            #[inline]
            fn one()  -> Self { 1 }
            #[inline]
            fn max()  -> Self { <$t>::MAX }
            #[inline]
            fn size() -> usize  { $size }
        }
    }
}


impl_bfcell!(Cell8, 8);
impl_bfcell!(Cell16, 16);
impl_bfcell!(Cell32, 32);
impl_bfcell!(Cell64, 64);
impl_bfcell!(Cell128, 128);

