mod original;


pub use original::*;

use core::ops::{
    Add,
    Sub,
};


pub trait BFCell<Rhs = Self, Output = Self>:
    Copy +
    Eq +
    Add<Rhs, Output = Output> +
    Sub<Rhs, Output = Output> {


    fn overflowing_add(self, rhs: Rhs) -> (Output, bool);
    fn overflowing_sub(self, rhs: Rhs) -> (Output, bool);

    fn saturating_add(self, rhs: Rhs) -> Output;
    fn saturating_sub(self, rhs: Rhs) -> Output;

    fn wrapping_add(self, rhs: Rhs) -> Output;
    fn wrapping_sub(self, rhs: Rhs) -> Output;

    fn zero() -> Output;
    fn one()  -> Output;
    fn min()  -> Output { Self::zero() }
    fn max()  -> Output;
    fn size() -> usize;
}

