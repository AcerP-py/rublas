use std::ops::Mul;

use crate::kernel::k_scal;

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// scal (x = ax)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `alpha` - scaling factor for `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
pub fn scal<T>(n: usize, alpha: T, x: &mut [T], inc_x: usize) -> ()
where
    T: Copy + Mul<Output = T>,
{
    if n <= 0 {
        return ();
    }

    k_scal(n, alpha, x, inc_x);
}
