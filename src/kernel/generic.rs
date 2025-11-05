use std::ops::Mul;

use crate::utils::types::{CFLOAT, DFLOAT, SFLOAT, ZFLOAT};

pub mod templates;
pub mod scal;

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
        templates::mk_axpy_s8el(c, alpha, x, y);
    } else {
        c = n & !1;
        templates::mk_axpy_ns2el(c, alpha, x, inc_x, y, inc_y);
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
        templates::mk_axpy_s8el(c, alpha, x, y);
    } else {
        c = n & !1;
        templates::mk_axpy_ns2el(c, alpha, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] += alpha * x[i * inc_x];
    }
}

/// ak (architecture-kernel) _
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
pub fn ak_caxpy(
    n: usize,
    alpha: CFLOAT,
    x: &[CFLOAT],
    inc_x: usize,
    y: &mut [CFLOAT],
    inc_y: usize,
) -> () {
    let c: usize;

    if n < 1 {
        return ();
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        templates::mk_axpy_s8el(c, alpha, x, y);
    } else {
        c = n & !1;
        templates::mk_axpy_ns2el(c, alpha, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] += alpha * x[i * inc_x];
    }
}

/// ak (architecture-kernel) _
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
pub fn ak_zaxpy(
    n: usize,
    alpha: ZFLOAT,
    x: &[ZFLOAT],
    inc_x: usize,
    y: &mut [ZFLOAT],
    inc_y: usize,
) -> () {
    let c: usize;

    if n < 1 {
        return ();
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        templates::mk_axpy_s8el(c, alpha, x, y);
    } else {
        c = n & !1;
        templates::mk_axpy_ns2el(c, alpha, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] += alpha * x[i * inc_x];
    }
}

// ===========================================================================
// scal
// ===========================================================================

/// ak (architecture-kernel) _
/// scal (y = ay)
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
pub fn ak_scal<T: Copy + Mul>(n: usize, alpha: T, y: &mut [T], inc_y: usize) -> ()
where
    T: Mul<Output = T>,
{
    let c: usize;

    if n < 1 {
        return ();
    }

    if inc_y == 1 {
        c = n & !7;
        scal::mk_scal_s8el(c, alpha, y);
    } else {
        c = n & !1;
        scal::mk_scal_ns2el(c, alpha, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] = alpha * y[i * inc_y];
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
        result += templates::mk_dot_s8el(c, x, y);
    } else {
        c = n & !1;
        result += templates::mk_dot_ns2el(c, x, inc_x, y, inc_y);
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
        result += templates::mk_dot_s8el(c, x, y);
    } else {
        c = n & !1;
        result += templates::mk_dot_ns2el(c, x, inc_x, y, inc_y);
    }

    for i in c..n {
        result += x[i * inc_x] * y[i * inc_y];
    }

    result
}

// ===========================================================================
// ===========================================================================
// tests
// ===========================================================================
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ak_scal_inc1_multiple_of_8() {
        let mut y: Vec<usize> = vec![1; 8];
        ak_scal(8, 2, &mut y, 1);
        assert_eq!(y, vec![2; 8]);
    }

    #[test]
    fn test_ak_scal_inc1_not_multiple_of_8() {
        let mut y: Vec<usize> = vec![1; 11];
        ak_scal(11, 2, &mut y, 1);
        assert_eq!(y, vec![2; 11]);
    }

    #[test]
    fn test_ak_scal_inc2_multiple_of_2() {
        let mut y: Vec<usize> = vec![1, 99, 2, 99, 3, 99, 4, 99];
        ak_scal(4, 2, &mut y, 2);
        assert_eq!(y, vec![2, 99, 4, 99, 6, 99, 8, 99]);
    }

    #[test]
    fn test_ak_scal_inc2_not_multiple_of_2() {
        let mut y: Vec<usize> = vec![1, 99, 2, 99, 3, 99, 4, 99, 5, 99];
        ak_scal(5, 2, &mut y, 2);
        assert_eq!(y, vec![2, 99, 4, 99, 6, 99, 8, 99, 10, 99]);
    }
}
