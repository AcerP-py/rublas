use std::ops::{Add, AddAssign, Mul};

use crate::utils::types::Zeroable;

/// mk (micro-kernel) _
/// axpy (y = y + ax) _
/// ns2el (non-sequential 2 element)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 2 and within bounds).
///
/// * `alpha` - scaling factor for `x`.
///
/// * `x` - A vector.
///
/// * `inc_x` - Increment between elements of `x`.
///
/// * `y` - A vector.
///
/// * `inc_y` - Increment between elements of `y`.
pub fn mk_axpy_ns2el<T: AddAssign + Copy + Mul>(
    n: usize,
    alpha: T,
    x: &[T],
    inc_x: usize,
    y: &mut [T],
    inc_y: usize,
) -> ()
where
    T: Mul<Output = T>,
{
    let mut x1: usize = 0;
    let mut y1: usize = 0;
    let mut x2: usize = n / 2 * inc_x;
    let mut y2: usize = n / 2 * inc_y;

    for _ in 0..(n / 2) {
        y[y1] += alpha * x[x1];
        y[y2] += alpha * x[x2];
        x1 += inc_x;
        x2 += inc_x;
        y1 += inc_y;
        y2 += inc_y;
    }
}

/// mk (micro-kernel) _
/// axpy (y = y + ax) _
/// s8el (sequential 8 element)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 8 and within bounds).
///
/// * `alpha` - scaling factor for `x`.
///
/// * `x` - A vector.
///
/// * `y` - A vector.
pub fn mk_axpy_s8el<T: AddAssign + Copy + Mul>(n: usize, alpha: T, x: &[T], y: &mut [T]) -> ()
where
    T: Mul<Output = T>,
{
    for i in (0..n).step_by(8) {
        y[i] += alpha * x[i];
        y[i + 1] += alpha * x[i + 1];
        y[i + 2] += alpha * x[i + 2];
        y[i + 3] += alpha * x[i + 3];
        y[i + 4] += alpha * x[i + 4];
        y[i + 5] += alpha * x[i + 5];
        y[i + 6] += alpha * x[i + 6];
        y[i + 7] += alpha * x[i + 7];
    }
}

/// mk (micro-kernel) _
/// dot (dot product) _
/// ns2el (non-sequential 2 element)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 2 and within bounds).
///
/// * `x` - A vector.
///
/// * `inc_x` - Increment between elements of `x`.
///
/// * `y` - A vector.
///
/// * `inc_y` - Increment between elements of `y`.
///
/// # Returns
///
/// The dot product of `x` and `y`.
pub fn mk_dot_ns2el<T: Add + AddAssign + Copy + Mul + Zeroable>(
    n: usize,
    x: &[T],
    inc_x: usize,
    y: &[T],
    inc_y: usize,
) -> T
where
    T: Add<Output = T> + Mul<Output = T>,
{
    let mut result: T = T::zero();

    let mut x1: usize = 0;
    let mut y1: usize = 0;
    let mut x2: usize = n / 2 * inc_x;
    let mut y2: usize = n / 2 * inc_y;

    for _ in 0..(n / 2) {
        result += x[x1] * y[y1] + x[x2] * y[y2];
        x1 += inc_x;
        x2 += inc_x;
        y1 += inc_y;
        y2 += inc_y;
    }

    result
}

/// mk (micro-kernel) _
/// dot (dot product) _
/// s8el (sequential 8 element)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 8 and within bounds).
///
/// * `x` - A vector.
///
/// * `y` - A vector.
///
/// # Returns
///
/// The dot product of `x` and `y`.
pub fn mk_dot_s8el<T: Add + AddAssign + Copy + Mul + Zeroable>(n: usize, x: &[T], y: &[T]) -> T
where
    T: Add<Output = T> + Mul<Output = T>,
{
    let mut result: T = T::zero();

    for i in (0..n).step_by(8) {
        result += x[i] * y[i]
            + x[i + 1] * y[i + 1]
            + x[i + 2] * y[i + 2]
            + x[i + 3] * y[i + 3]
            + x[i + 4] * y[i + 4]
            + x[i + 5] * y[i + 5]
            + x[i + 6] * y[i + 6]
            + x[i + 7] * y[i + 7];
    }

    result
}
