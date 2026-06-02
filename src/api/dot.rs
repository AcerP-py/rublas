use std::ops::{Add, AddAssign, Mul};

use crate::{
    kernel::k_dot,
    utils::traits::{Conjigatable, Zeroable},
};

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
///
/// # Returns
/// the dot pruduct of vectors 'x' and 'y'
pub fn dot<T, A>(n: usize, x: &[T], inc_x: usize, y: &[T], inc_y: usize) -> A
where
    T: Add<Output = T> + Conjigatable + Copy + Mul<Output = T>,
    A: AddAssign + From<T> + Zeroable,
{
    if n <= 0 {
        return A::zero();
    }

    k_dot(n, x, inc_x, y, inc_y, false)
}

/// dotc (conj(x)^T * y)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
///
/// # Returns
/// the dot pruduct of vectors 'conjigated x' and 'y'
pub fn dotc<T, A>(n: usize, x: &[T], inc_x: usize, y: &[T], inc_y: usize) -> A
where
    T: Add<Output = T> + Conjigatable + Copy + Mul<Output = T>,
    A: AddAssign + From<T> + Zeroable,
{
    if n <= 0 {
        return A::zero();
    }

    k_dot(n, x, inc_x, y, inc_y, true)
}
