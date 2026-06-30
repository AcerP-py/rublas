use crate::utils::traits::{BLASAbsoluteValue, Conjugatable, Zeroable};
use cfg_if::cfg_if;
use std::ops::{Add, AddAssign, Mul, Neg};

// ***************************************************************************
// ***************************************************************************
// architecture kernel modules
// ***************************************************************************
// ***************************************************************************

// allow dead code in generic so it can act as a backup for architectures
// without an implementation
#[allow(dead_code)]
mod generic;

// other architectures
cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
    }
}

// ***************************************************************************
// ***************************************************************************
// BLAS kernels
// ***************************************************************************
// ***************************************************************************

// ===========================================================================
// asum
// ===========================================================================

/// k (kernel) _
/// asum (sum of "absolute" (|Re(x)|+|Im(x)|) values)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
///
/// # Returns
/// the sum of absolute values of vector elements
pub fn k_asum<T, A>(n: usize, x: &[T], inc_x: usize) -> A
where
    T: BLASAbsoluteValue<Output = A> + Copy,
    A: AddAssign + Zeroable,
{
    generic::ak_asum(n, x, inc_x)
}

// ===========================================================================
// axpy
// ===========================================================================

/// k (kernel) _
/// axpy (y = y + ax)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `alpha` - scaling factor for `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
pub fn k_axpy<T>(n: usize, alpha: T, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: AddAssign + Copy + Mul<Output = T>,
{
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::ak_axpy(n, alpha, x, inc_x, y, inc_y);
        } else {
            generic::ak_axpy(n, alpha, x, inc_x, y, inc_y);
        }
    }
}

// ===========================================================================
// copy
// ===========================================================================

/// k (kernel) _
/// copy (y = x)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
pub fn k_copy<T>(n: usize, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: Copy,
{
    generic::ak_copy(n, x, inc_x, y, inc_y);
}

// ===========================================================================
// dot
// ===========================================================================

/// k (kernel) _
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
pub fn k_dot<T, A>(n: usize, x: &[T], inc_x: usize, y: &[T], inc_y: usize, conjugate: bool) -> A
where
    T: Add<Output = T> + Conjugatable + Copy + Mul<Output = T>,
    A: AddAssign + From<T> + Zeroable,
{
    generic::ak_dot(n, x, inc_x, y, inc_y, conjugate)
}

// ===========================================================================
// iamax
// ===========================================================================

/// k (kernel) _
/// iamax (the element with the largest "absolute" (|Re(x)|+|Im(x)|) value)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
///
/// # Returns
/// the index of the first element with the largest absolute value
pub fn k_iamax<T>(n: usize, x: &[T], inc_x: usize) -> usize
where
    T: BLASAbsoluteValue + Copy,
{
    generic::ak_iamax(n, x, inc_x)
}

// ===========================================================================
// rot
// ===========================================================================

/// k (kernel) _
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
pub fn k_rot<T, A, B>(
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
    generic::ak_rot(n, x, inc_x, y, inc_y, c, s);
}

// ===========================================================================
// scal
// ===========================================================================

/// k (kernel) _
/// scal (x = ax)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `alpha` - scaling factor for `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
pub fn k_scal<T>(n: usize, alpha: T, x: &mut [T], inc_x: usize) -> ()
where
    T: Copy + Mul<Output = T>,
{
    generic::ak_scal(n, alpha, x, inc_x);
}

// ===========================================================================
// swap
// ===========================================================================

/// k (kernel) _
/// swap (x <-> y)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
pub fn k_swap<T>(n: usize, x: &mut [T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: Copy + Zeroable,
{
    generic::ak_swap(n, x, inc_x, y, inc_y);
}
