use crate::{
    kernel::k_asum,
    utils::traits::{BLASAbsoluteValue, Zeroable},
};
use std::ops::AddAssign;

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// asum (sum of "absolute" (|Re(x)|+|Im(x)|) values)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
///
/// # Returns
/// the sum of absolute values of vector elements
pub fn asum<T, A>(n: usize, x: &[T], inc_x: usize) -> A
where
    T: BLASAbsoluteValue<Output = A> + Copy,
    A: AddAssign + Zeroable,
{
    if n <= 0 {
        return A::zero();
    }

    k_asum(n, x, inc_x)
}
