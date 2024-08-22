use super::*;


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


impl_bfcell!(u8, 8);
impl_bfcell!(u16, 16);
impl_bfcell!(u32, 32);
impl_bfcell!(u64, 64);
impl_bfcell!(u128, 128);

