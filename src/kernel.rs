use cfg_if::cfg_if;

use crate::utils::types::{CFLOAT, DFLOAT, SFLOAT, ZFLOAT};

// ===========================================================================
// ===========================================================================
// architecture modules
// ===========================================================================
// ===========================================================================

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

// ===========================================================================
// ===========================================================================
// dynamic BLAS kernels
// ===========================================================================
// ===========================================================================

// ===========================================================================
// axpy
// ===========================================================================

/// k (kernel) _
/// saxpy (SFLOAT y = y + ax)
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
pub fn k_saxpy(
    n: usize,
    alpha: SFLOAT,
    x: &[SFLOAT],
    inc_x: usize,
    y: &mut [SFLOAT],
    inc_y: usize,
) -> () {
    generic::ak_saxpy(n, alpha, x, inc_x, y, inc_y);
}

/// k (kernel) _
/// daxpy (DFLOAT y = y + ax)
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
pub fn k_daxpy(
    n: usize,
    alpha: DFLOAT,
    x: &[DFLOAT],
    inc_x: usize,
    y: &mut [DFLOAT],
    inc_y: usize,
) -> () {
    generic::ak_daxpy(n, alpha, x, inc_x, y, inc_y);
}

/// k (kernel) _
/// caxpy (CFLOAT y = y + ax)
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
pub fn k_caxpy(
    n: usize,
    alpha: CFLOAT,
    x: &[CFLOAT],
    inc_x: usize,
    y: &mut [CFLOAT],
    inc_y: usize,
) -> () {
    generic::ak_caxpy(n, alpha, x, inc_x, y, inc_y);
}

/// k (kernel) _
/// zaxpy (ZFLOAT y = y + ax)
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
pub fn k_zaxpy(
    n: usize,
    alpha: ZFLOAT,
    x: &[ZFLOAT],
    inc_x: usize,
    y: &mut [ZFLOAT],
    inc_y: usize,
) -> () {
    generic::ak_zaxpy(n, alpha, x, inc_x, y, inc_y);
}

// ===========================================================================
// dot
// ===========================================================================

/// k (kernel) _
/// sdot (SFLOAT dot product)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`.
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
pub fn k_sdot(n: usize, x: &[SFLOAT], inc_x: usize, y: &[SFLOAT], inc_y: usize) -> SFLOAT {
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::ak_sdot(n, x, inc_x, y, inc_y)
        } else {
            generic::ak_sdot(n, x, inc_x, y, inc_y)
        }
    }
}

/// k (kernel) _
/// ddot (DFLOAT dot product)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`.
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
pub fn k_ddot(n: usize, x: &[DFLOAT], inc_x: usize, y: &[DFLOAT], inc_y: usize) -> DFLOAT {
    generic::ak_ddot(n, x, inc_x, y, inc_y)
}
