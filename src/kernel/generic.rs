use std::ops::{AddAssign, Mul};

pub mod axpy;
pub mod scal;

// ***************************************************************************
// ***************************************************************************
// architecture kernels
// ***************************************************************************
// ***************************************************************************

// ===========================================================================
// axpy
// ===========================================================================

/// ak (architecture-kernel) _
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
pub fn ak_axpy<T>(n: usize, alpha: T, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: AddAssign + Copy + Mul<Output = T> + axpy::AXPY,
{
    let c: usize;

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        T::mk_axpy_8sel(c, alpha, x, y);
    } else {
        c = n & !1;
        T::mk_axpy_2nsel(c, alpha, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] += alpha * x[i * inc_x];
    }
}

// ===========================================================================
// scal
// ===========================================================================

/// ak (architecture-kernel) _
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
pub fn ak_scal<T>(n: usize, alpha: T, x: &mut [T], inc_x: usize) -> ()
where
    T: Copy + Mul<Output = T> + scal::SCAL,
{
    let c: usize;

    if inc_x == 1 {
        c = n & !7;
        T::mk_scal_8sel(c, alpha, x);
    } else {
        c = n & !1;
        T::mk_scal_2nsel(c, alpha, x, inc_x);
    }

    for i in c..n {
        x[i * inc_x] = alpha * x[i * inc_x];
    }
}

// ***************************************************************************
// ***************************************************************************
// tests
// ***************************************************************************
// ***************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ak_axpy_basic() {
        let x: Vec<usize> = vec![1; 10];
        let mut y: Vec<usize> = vec![1; 10];

        ak_axpy(10, 2, &x, 1, &mut y, 1);
        assert_eq!(y, vec![3; 10]);

        y = vec![1; 10];
        ak_axpy(5, 2, &x, 2, &mut y, 2);
        assert_eq!(y, Vec::from([3, 1, 3, 1, 3, 1, 3, 1, 3, 1]))
    }

    #[test]
    fn ak_scal_basic() {
        let mut x: Vec<usize> = vec![1; 10];

        ak_scal(10, 2, &mut x, 1);
        assert_eq!(x, vec![2; 10]);

        x = vec![1; 10];
        ak_scal(5, 2, &mut x, 2);
        assert_eq!(x, Vec::from([2, 1, 2, 1, 2, 1, 2, 1, 2, 1]))
    }
}
