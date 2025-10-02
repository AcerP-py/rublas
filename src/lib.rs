// Copyright 2025 Asa Rentschler.

//! A pure Rust implementation of BLAS!
//!
//! It generally follows <https://www.netlib.org/blas/cblas.h> but
//! does differ in a few places where it makes sense.
//!
//! # Usage
//! ``` rust
//! use rublas::*;
//!
//! const N: usize = 1000;
//!
//! fn main() {
//!     let x: Vec<SFLOAT> = vec![1.5; N];
//!     let y: Vec<SFLOAT> = vec![2.7; N];
//!
//!     let result: SFLOAT = sdot(N, &x, 1, &y, 1);
//!
//!     println!("Result: {}", result);
//! }
//! ```

mod utils;
mod kernel;

pub use utils::types::{CFLOAT, DFLOAT, SFLOAT, ZFLOAT};

// ===========================================================================
// ===========================================================================
// Level 1 BLAS
// ===========================================================================
// ===========================================================================

// ===========================================================================
// axpy
// ===========================================================================

/// Performs `y = y + ax` on two single-precision vectors.
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
pub fn saxpy(
    n: usize,
    alpha: SFLOAT,
    x: &[SFLOAT],
    inc_x: usize,
    y: &mut [SFLOAT],
    inc_y: usize,
) -> () {
    kernel::k_saxpy(n, alpha, x, inc_x, y, inc_y);
}

/// Performs `y = y + ax` on two double-precision vectors.
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
pub fn daxpy(
    n: usize,
    alpha: DFLOAT,
    x: &[DFLOAT],
    inc_x: usize,
    y: &mut [DFLOAT],
    inc_y: usize,
) -> () {
    kernel::k_daxpy(n, alpha, x, inc_x, y, inc_y);
}

/// Performs `y = y + ax` on two single-precision complex vectors.
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
pub fn caxpy(
    n: usize,
    alpha: CFLOAT,
    x: &[CFLOAT],
    inc_x: usize,
    y: &mut [CFLOAT],
    inc_y: usize,
) -> () {
    kernel::k_caxpy(n, alpha, x, inc_x, y, inc_y);
}

/// Performs `y = y + ax` on two double-precision complex vectors.
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
pub fn zaxpy(
    n: usize,
    alpha: ZFLOAT,
    x: &[ZFLOAT],
    inc_x: usize,
    y: &mut [ZFLOAT],
    inc_y: usize,
) -> () {
    kernel::k_zaxpy(n, alpha, x, inc_x, y, inc_y);
}

// ===========================================================================
// scal
// ===========================================================================

/// Performs `y = ay` on a single-precision vector.
///
/// # Arguments
///
/// * `n` - Length of vector `y`.
///
/// * `alpha` - scaling factor for `y`.
///
/// * `y` - A vector.
///
/// * `inc_y` - Increment between elements of `y`.
pub fn sscal(n: usize, alpha: SFLOAT, y: &mut [SFLOAT], inc_y: usize) -> () {
    kernel::k_sscal(n, alpha, y, inc_y);
}

// ===========================================================================
// dot
// ===========================================================================

/// The dot product of two single-precision vectors.
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
pub fn sdot(n: usize, x: &[SFLOAT], inc_x: usize, y: &[SFLOAT], inc_y: usize) -> SFLOAT {
    kernel::k_sdot(n, x, inc_x, y, inc_y)
}

/// The dot product of two double-precision vectors.
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
pub fn ddot(n: usize, x: &[DFLOAT], inc_x: usize, y: &[DFLOAT], inc_y: usize) -> DFLOAT {
    kernel::k_ddot(n, x, inc_x, y, inc_y)
}
