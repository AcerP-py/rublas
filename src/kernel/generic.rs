// ===========================================================================
// ===========================================================================
// operation modules
// ===========================================================================
// ===========================================================================

// ===========================================================================
// dot
// ===========================================================================

mod saxpy;

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
pub fn ak_saxpy(n: usize, alpha: f32, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () {
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
        y[i * inc_y] = y[i * inc_y] + alpha * x[i * inc_x];
    }
}

// ===========================================================================
// dot
// ===========================================================================

/// ak (architecture-kernel) _
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
pub fn ak_sdot(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 {
    let mut result: f32 = 0.0;
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
pub fn ak_ddot(n: usize, x: &[f64], inc_x: usize, y: &[f64], inc_y: usize) -> f64 {
    let mut result: f64 = 0.0;
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
