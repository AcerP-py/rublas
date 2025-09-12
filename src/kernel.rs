use cfg_if::cfg_if;

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
/// saxpy (f32 y = y + ax)
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
pub fn k_saxpy(n: usize, alpha: f32, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () {
    generic::ak_saxpy(n, alpha, x, inc_x, y, inc_y);
}

// ===========================================================================
// dot
// ===========================================================================

/// k (kernel) _
/// sdot (f32 dot product)
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
pub fn k_sdot(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 {
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::ak_sdot(n, x, inc_x, y, inc_y)
        } else {
            generic::ak_sdot(n, x, inc_x, y, inc_y)
        }
    }
}

/// k (kernel) _
/// ddot (f64 dot product)
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
pub fn k_ddot(n: usize, x: &[f64], inc_x: usize, y: &[f64], inc_y: usize) -> f64 {
    generic::ak_ddot(n, x, inc_x, y, inc_y)
}
