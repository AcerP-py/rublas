use crate::kernel::k_rot;
use std::ops::{Add, Mul, Neg};

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// rot (apply a Givens rotation)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
/// * `c` - cosine factor
/// * `s` - sine factor
pub fn rot<T, A, B>(
    n: usize,
    x: &mut [T],
    inc_x: usize,
    y: &mut [T],
    inc_y: usize,
    c: A,
    s: B,
) -> ()
where
    T: Add<Output = T> + Copy,
    A: Copy + Mul<T, Output = T>,
    B: Copy + Mul<T, Output = T> + Neg<Output = B>,
{
    if n == 0 {
        return ();
    }

    k_rot(n, x, inc_x, y, inc_y, c, s);
}
