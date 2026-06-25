use crate::kernel::k_axpy;
use std::ops::{AddAssign, Mul};

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// axpy (y = y + ax)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `alpha` - scaling factor for `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
pub fn axpy<T>(n: usize, alpha: T, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: AddAssign + Copy + Mul<Output = T>,
{
    if n <= 0 {
        return ();
    }

    k_axpy(n, alpha, x, inc_x, y, inc_y);
}
