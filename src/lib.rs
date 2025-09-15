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
//!     let x: Vec<f32> = vec![1.5; N];
//!     let y: Vec<f32> = vec![2.7; N];
//!
//!     let result: f32 = sdot(N, &x, 1, &y, 1);
//!
//!     println!("Result: {}", result);
//! }
//! ```

mod kernel;

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
pub fn saxpy(n: usize, alpha: f32, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () {
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
pub fn daxpy(n: usize, alpha: f64, x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> () {
    kernel::k_daxpy(n, alpha, x, inc_x, y, inc_y);
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
pub fn sdot(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 {
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
pub fn ddot(n: usize, x: &[f64], inc_x: usize, y: &[f64], inc_y: usize) -> f64 {
    kernel::k_ddot(n, x, inc_x, y, inc_y)
}
