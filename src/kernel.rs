use std::ops::{AddAssign, Mul};

use cfg_if::cfg_if;

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
// axpy
// ===========================================================================

/// k (kernel) _
/// axpy (y = y + ax)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`.
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
// scal
// ===========================================================================

/// k (kernel) _
/// scal (x = ax)
///
/// # Arguments
///
/// * `n` - Length of vector `x`.
///
/// * `alpha` - scaling factor for `x`.
///
/// * `x` - A vector.
///
/// * `inc_x` - Increment between elements of `x`.
pub fn k_scal<T>(n: usize, alpha: T, x: &mut [T], inc_x: usize) -> ()
where
    T: Copy + Mul<Output = T>,
{
    generic::ak_scal(n, alpha, x, inc_x);
}
