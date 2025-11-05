use std::ops::Mul;

// ===========================================================================
// ===========================================================================
// template kernels
// ===========================================================================
// ===========================================================================

/// mk (micro-kernel) _
/// scal (y = ay) _
/// ns2el (non-sequential 2 element)
///
/// # Arguments
///
/// * `n` - Length of vector `y`
/// (must be divisible by 2 and within bounds).
///
/// * `alpha` - scaling factor for `y`.
///
/// * `y` - A vector.
///
/// * `inc_y` - Increment between elements of `y`.
pub fn mk_scal_ns2el<T: Copy + Mul>(n: usize, alpha: T, y: &mut [T], inc_y: usize) -> ()
where
    T: Mul<Output = T>,
{
    let mut y1: usize = 0;
    let mut y2: usize = n / 2 * inc_y;

    for _ in 0..(n / 2) {
        y[y1] = alpha * y[y1];
        y[y2] = alpha * y[y2];
        y1 += inc_y;
        y2 += inc_y;
    }
}

/// mk (micro-kernel) _
/// scal (y = ay) _
/// s8el (sequential 8 element)
///
/// # Arguments
///
/// * `n` - Length of vector `y`
/// (must be divisible by 8 and within bounds).
///
/// * `alpha` - scaling factor for `y`.
///
/// * `y` - A vector.
pub fn mk_scal_s8el<T: Copy + Mul>(n: usize, alpha: T, y: &mut [T]) -> ()
where
    T: Mul<Output = T>,
{
    for i in (0..n).step_by(8) {
        y[i] = alpha * y[i];
        y[i + 1] = alpha * y[i + 1];
        y[i + 2] = alpha * y[i + 2];
        y[i + 3] = alpha * y[i + 3];
        y[i + 4] = alpha * y[i + 4];
        y[i + 5] = alpha * y[i + 5];
        y[i + 6] = alpha * y[i + 6];
        y[i + 7] = alpha * y[i + 7];
    }
}

// ===========================================================================
// ===========================================================================
// specific kernels
// ===========================================================================
// ===========================================================================

// ===========================================================================
// ===========================================================================
// tests
// ===========================================================================
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mk_scal_ns2el_basic() {
        let mut y: Vec<usize> = vec![1, 2, 3, 4];
        let inc_y: usize = 1;
        mk_scal_ns2el(4, 2, &mut y, inc_y);
        assert_eq!(y, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_mk_scal_ns2el_stride_2() {
        let mut y: Vec<usize> = vec![1, 99, 2, 99, 3, 99, 4, 99];
        let inc_y: usize = 2;
        mk_scal_ns2el(4, 2, &mut y, inc_y);
        assert_eq!(y, vec![2, 99, 4, 99, 6, 99, 8, 99]);
    }

    #[test]
    fn test_mk_scal_s8el_basic() {
        let mut y: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        mk_scal_s8el(8, 2, &mut y);
        assert_eq!(y, vec![2, 4, 6, 8, 10, 12, 14, 16]);
    }

    #[test]
    #[should_panic]
    fn test_mk_scal_ns2el_invalid_bounds() {
        let mut y: Vec<usize> = vec![1; 2];
        mk_scal_ns2el(4, 2, &mut y, 1);
    }

    #[test]
    #[should_panic]
    fn test_mk_scal_s8el_invalid_bounds() {
        let mut y: Vec<usize> = vec![1; 4];
        mk_scal_s8el(8, 2, &mut y);
    }
}
