use crate::{
    kernel::k_dot,
    utils::traits::{Conjugatable, Zeroable},
};
use std::ops::{Add, AddAssign, Mul};

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// dot (x^T * y)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
/// * `conjugate` - conjugate `x` before calculations
///
/// # Returns
/// the dot product of vectors 'x' and 'y'
pub fn dot<T, A>(n: usize, x: &[T], inc_x: usize, y: &[T], inc_y: usize, conjugate: bool) -> A
where
    T: Add<Output = T> + Conjugatable + Copy + Mul<Output = T>,
    A: AddAssign + From<T> + Zeroable,
{
    if n == 0 {
        return A::zero();
    }

    k_dot(n, x, inc_x, y, inc_y, conjugate)
}
