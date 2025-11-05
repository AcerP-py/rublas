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

use utils::types::Zeroable;

pub use utils::types::{CFLOAT, DFLOAT, SFLOAT, ZFLOAT};

// ***************************************************************************
// ***************************************************************************
// Level 1 BLAS
// ***************************************************************************
// ***************************************************************************

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
    if n <= 0 {
        return ();
    }

    kernel::k_saxpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_saxpy(
    n: i32,
    alpha: f32,
    x: *const f32,
    inc_x: i32,
    y: *mut f32,
    inc_y: i32,
) -> () {
    let n: usize = n as usize;
    let alpha: SFLOAT = alpha as SFLOAT;
    let inc_x: usize = inc_x as usize;
    let inc_y: usize = inc_y as usize;

    unsafe {
        let x_slice: &[SFLOAT] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [SFLOAT] = std::slice::from_raw_parts_mut(y, n * inc_y);
        saxpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
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
    if n <= 0 {
        return ();
    }

    kernel::k_daxpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_daxpy(
    n: i32,
    alpha: f64,
    x: *const f64,
    inc_x: i32,
    y: *mut f64,
    inc_y: i32,
) -> () {
    let n: usize = n as usize;
    let alpha: DFLOAT = alpha as DFLOAT;
    let inc_x: usize = inc_x as usize;
    let inc_y: usize = inc_y as usize;

    unsafe {
        let x_slice: &[DFLOAT] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [DFLOAT] = std::slice::from_raw_parts_mut(y, n * inc_y);
        daxpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
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
    if n <= 0 {
        return ();
    }

    kernel::k_caxpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_caxpy(
    n: i32,
    alpha: *const f32,
    x: *const f32,
    inc_x: i32,
    y: *mut f32,
    inc_y: i32,
) -> () {
    let n = n as usize;
    let alpha: CFLOAT = CFLOAT::from(alpha);
    let inc_x = inc_x as usize;
    let inc_y = inc_y as usize;
    let x: *const CFLOAT = x as *const CFLOAT;
    let y: *mut CFLOAT = y as *mut CFLOAT;

    unsafe {
        let x_slice: &[CFLOAT] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [CFLOAT] = std::slice::from_raw_parts_mut(y, n * inc_y);
        caxpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
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
    if n <= 0 {
        return ();
    }

    kernel::k_zaxpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_zaxpy(
    n: i32,
    alpha: *const f64,
    x: *const f64,
    inc_x: i32,
    y: *mut f64,
    inc_y: i32,
) -> () {
    let n = n as usize;
    let alpha: ZFLOAT = ZFLOAT::from(alpha);
    let inc_x = inc_x as usize;
    let inc_y = inc_y as usize;
    let x: *const ZFLOAT = x as *const ZFLOAT;
    let y: *mut ZFLOAT = y as *mut ZFLOAT;

    unsafe {
        let x_slice: &[ZFLOAT] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [ZFLOAT] = std::slice::from_raw_parts_mut(y, n * inc_y);
        zaxpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
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
    if n <= 0 {
        return ();
    }

    kernel::k_sscal(n, alpha, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_sscal(n: i32, alpha: f32, y: *mut f32, inc_y: i32) -> () {
    let n: usize = n as usize;
    let alpha: SFLOAT = alpha as SFLOAT;
    let inc_y: usize = inc_y as usize;

    unsafe {
        let y_slice: &mut [SFLOAT] = std::slice::from_raw_parts_mut(y, n * inc_y);
        sscal(n, alpha, y_slice, inc_y);
    }
}

/// Performs `y = ay` on a double-precision vector.
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
pub fn dscal(n: usize, alpha: DFLOAT, y: &mut [DFLOAT], inc_y: usize) -> () {
    if n <= 0 {
        return ();
    }

    kernel::k_dscal(n, alpha, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_dscal(n: i32, alpha: f64, y: *mut f64, inc_y: i32) -> () {
    let n: usize = n as usize;
    let alpha: DFLOAT = alpha as DFLOAT;
    let inc_y: usize = inc_y as usize;

    unsafe {
        let y_slice: &mut [DFLOAT] = std::slice::from_raw_parts_mut(y, n * inc_y);
        dscal(n, alpha, y_slice, inc_y);
    }
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
    if n <= 0 {
        return SFLOAT::zero();
    }

    kernel::k_sdot(n, x, inc_x, y, inc_y)
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_sdot(
    n: i32,
    x: *const f32,
    inc_x: i32,
    y: *const f32,
    inc_y: i32,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let inc_y: usize = inc_y as usize;

    unsafe {
        let x_slice: &[SFLOAT] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &[SFLOAT] = std::slice::from_raw_parts(y, n * inc_y);
        sdot(n, x_slice, inc_x, y_slice, inc_y);
    }
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
    if n <= 0 {
        return DFLOAT::zero();
    }

    kernel::k_ddot(n, x, inc_x, y, inc_y)
}

#[unsafe(no_mangle)]
pub extern "C" fn _extern_c_ddot(
    n: i32,
    x: *const f64,
    inc_x: i32,
    y: *const f64,
    inc_y: i32,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let inc_y: usize = inc_y as usize;

    unsafe {
        let x_slice: &[DFLOAT] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &[DFLOAT] = std::slice::from_raw_parts(y, n * inc_y);
        ddot(n, x_slice, inc_x, y_slice, inc_y);
    }
}
