use crate::{
    kernel::k_iamax,
    utils::traits::{BLASAbsoluteValue, Zeroable},
};

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// iamax (the element with the largest "absolute" (|Re(x)|+|Im(x)|) value)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
///
/// # Returns
/// the index of the first element with the largest absolute value
pub fn iamax<T>(n: usize, x: &[T], inc_x: usize) -> usize
where
    T: BLASAbsoluteValue + Copy,
{
    if n <= 0 {
        return usize::zero();
    }

    k_iamax(n, x, inc_x)
}
