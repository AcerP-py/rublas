use crate::{kernel::k_rotm, utils::enums::RotmFlag};
use std::ops::{Add, Mul};

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// rotm (apply a modified Givens rotation)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
/// * `flag` - type of modified rotation to apply
/// * `params` - 4 element (2x2) transformation matrix
pub fn rotm<T>(
    n: usize,
    x: &mut [T],
    inc_x: usize,
    y: &mut [T],
    inc_y: usize,
    flag: RotmFlag,
    params: &[T],
) -> ()
where
    T: Add<Output = T> + Copy + Mul<Output = T>,
{
    if n <= 0 || flag == RotmFlag::Identity {
        return ();
    }

    k_rotm(n, x, inc_x, y, inc_y, flag, params);
}
