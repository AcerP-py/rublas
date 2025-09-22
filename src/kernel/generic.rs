use crate::utils::types::{DFLOAT, SFLOAT};

// ===========================================================================
// ===========================================================================
// operation modules
// ===========================================================================
// ===========================================================================

// ===========================================================================
// dot
// ===========================================================================

mod saxpy;
mod daxpy;

// ===========================================================================
// dot
// ===========================================================================

mod sdot;
mod ddot;

// ===========================================================================
// ===========================================================================
// architecture kernels
// ===========================================================================
// ===========================================================================

// ===========================================================================
// axpy
// ===========================================================================

/// ak (architecture-kernel) _
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
pub fn ak_saxpy(
    n: usize,
    alpha: SFLOAT,
    x: &[SFLOAT],
    inc_x: usize,
    y: &mut [SFLOAT],
    inc_y: usize,
) -> () {
    let c: usize;

    if n < 1 {
        return ();
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        saxpy::mk_saxpy_s8el(c, alpha, x, y);
    } else {
        c = n & !1;
        saxpy::mk_saxpy_ns2el(c, alpha, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] += alpha * x[i * inc_x];
    }
}

/// ak (architecture-kernel) _
/// saxpy (DFLOAT y = y + ax)
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
pub fn ak_daxpy(
    n: usize,
    alpha: DFLOAT,
    x: &[DFLOAT],
    inc_x: usize,
    y: &mut [DFLOAT],
    inc_y: usize,
) -> () {
    let c: usize;

    if n < 1 {
        return ();
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        daxpy::mk_daxpy_s8el(c, alpha, x, y);
    } else {
        c = n & !1;
        daxpy::mk_daxpy_ns2el(c, alpha, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] += alpha * x[i * inc_x];
    }
}

// ===========================================================================
// dot
// ===========================================================================

/// ak (architecture-kernel) _
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
pub fn ak_sdot(n: usize, x: &[SFLOAT], inc_x: usize, y: &[SFLOAT], inc_y: usize) -> SFLOAT {
    let mut result: SFLOAT = 0.0;
    let c: usize;

    if n < 1 {
        return result;
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        result += sdot::mk_sdot_s8el(c, x, y);
    } else {
        c = n & !1;
        result += sdot::mk_sdot_ns2el(c, x, inc_x, y, inc_y);
    }

    for i in c..n {
        result += x[i] * y[i];
    }

    result
}

/// ak (architecture-kernel) _
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
pub fn ak_ddot(n: usize, x: &[DFLOAT], inc_x: usize, y: &[DFLOAT], inc_y: usize) -> DFLOAT {
    let mut result: DFLOAT = 0.0;
    let c: usize;

    if n < 1 {
        return result;
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        result += ddot::mk_ddot_s8el(c, x, y);
    } else {
        c = n & !1;
        result += ddot::mk_ddot_ns2el(c, x, inc_x, y, inc_y);
    }

    for i in c..n {
        result += x[i * inc_x] * y[i * inc_y];
    }

    result
}
