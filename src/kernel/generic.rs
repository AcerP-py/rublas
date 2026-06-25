use crate::utils::traits::{Conjugatable, Zeroable};
use std::ops::{Add, AddAssign, Mul};

pub mod axpy;
pub mod copy;
pub mod dot;
pub mod iamax;
pub mod scal;
pub mod swap;

// ***************************************************************************
// ***************************************************************************
// level 1 architecture kernels
// ***************************************************************************
// ***************************************************************************

// ===========================================================================
// axpy
// ===========================================================================

/// ak (architecture-kernel) _
/// axpy (y = y + ax)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `alpha` - scaling factor for `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
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
// copy
// ===========================================================================

/// ak (architecture-kernel) _
/// copy (y = x)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
pub fn ak_copy<T>(n: usize, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: Copy + copy::COPY,
{
    let c: usize;

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        T::mk_copy_8sel(c, x, y);
    } else {
        c = n & !1;
        T::mk_copy_2nsel(c, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] = x[i * inc_x];
    }
}

// ===========================================================================
// swap
// ===========================================================================

/// ak (architecture-kernel) _
/// swap (x <-> y)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
pub fn ak_swap<T>(n: usize, x: &mut [T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: Copy + swap::SWAP,
{
    let c: usize;
    let mut shelf: T;

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        T::mk_swap_8sel(c, x, y);
    } else {
        c = n & !1;
        T::mk_swap_2nsel(c, x, inc_x, y, inc_y);
    }

    for i in c..n {
        shelf = y[i * inc_y];
        y[i * inc_y] = x[i * inc_x];
        x[i * inc_x] = shelf;
    }
}

// ===========================================================================
// dot
// ===========================================================================

/// ak (architecture-kernel) _
/// dot (x^T*y)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
/// * `conjugate` - conjugate `x` before calculations
///
/// # Returns
/// the dot product of vectors 'x' and 'y'
pub fn ak_dot<T, A>(n: usize, x: &[T], inc_x: usize, y: &[T], inc_y: usize, conjugate: bool) -> A
where
    T: Add<Output = T> + Conjugatable + Copy + Mul<Output = T> + dot::DOT<A>,
    A: AddAssign + From<T> + Zeroable,
{
    let c: usize;
    let mut result: A = A::zero();

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        if conjugate {
            result += T::mk_dot_conj_8sel(c, x, y);
        } else {
            result += T::mk_dot_8sel(c, x, y);
        }
    } else {
        c = n & !1;
        if conjugate {
            result += T::mk_dot_conj_2nsel(c, x, inc_x, y, inc_y);
        } else {
            result += T::mk_dot_2nsel(c, x, inc_x, y, inc_y);
        }
    }

    for i in c..n {
        if conjugate {
            result += A::from(x[i * inc_x].conj() * y[i * inc_y]);
        } else {
            result += A::from(x[i * inc_x] * y[i * inc_y]);
        }
    }

    result
}

// ===========================================================================
// iamax
// ===========================================================================

/// ak (architecture-kernel) _
/// iamax (the element with the largest "absolute" (|Re(x)|+|Im(x)|) value)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
///
/// # Returns
/// the index of the first element with the largest absolute value
pub fn ak_iamax<T>(n: usize, x: &[T], inc_x: usize) -> usize
where
    T: iamax::IAMAX,
{
    if inc_x == 1 {
        T::mk_iamax_sel(n, x)
    } else {
        T::mk_iamax_nsel(n, x, inc_x)
    }
}

// ===========================================================================
// scal
// ===========================================================================

/// ak (architecture-kernel) _
/// scal (x = ax)
///
/// # Arguments
/// * `n` - length of vector `x`
/// * `alpha` - scaling factor for `x`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
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
        let x: Vec<i32> = vec![1; 9];
        let mut y: Vec<i32>;
        let mut inc: usize;

        y = vec![2; 9];
        inc = 1;
        ak_axpy(9, 2, &x, inc, &mut y, inc);
        assert_eq!(y, vec![4; 9]);

        y = vec![2; 9];
        inc = 2;
        ak_axpy(5, 2, &x, inc, &mut y, inc);
        assert_eq!(y, vec![4, 2, 4, 2, 4, 2, 4, 2, 4]);
    }

    #[test]
    fn ak_copy_basic() {
        let x: Vec<i32> = vec![1; 9];
        let mut y: Vec<i32>;
        let mut inc: usize;

        y = vec![2; 9];
        inc = 1;
        ak_copy(9, &x, inc, &mut y, inc);
        assert_eq!(y, x);

        y = vec![2; 9];
        inc = 2;
        ak_copy(5, &x, inc, &mut y, inc);
        assert_eq!(y, vec![1, 2, 1, 2, 1, 2, 1, 2, 1]);
    }

    #[test]
    fn ak_swap_basic() {
        let mut x: Vec<i32> = vec![1; 9];
        let mut y: Vec<i32> = vec![2; 9];
        let mut inc: usize;

        inc = 1;
        ak_swap(9, &mut x, inc, &mut y, inc);
        assert_eq!(x, vec![2; 9]);
        assert_eq!(y, vec![1; 9]);

        inc = 2;
        ak_swap(5, &mut x, inc, &mut y, inc);
        assert_eq!(x, vec![1, 2, 1, 2, 1, 2, 1, 2, 1]);
        assert_eq!(y, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]);
    }

    #[test]
    fn ak_dot_basic() {
        let x: Vec<i32> = vec![1; 9];
        let mut y: Vec<i32> = vec![2; 9];
        let mut inc: usize;
        let mut res: i32;

        inc = 1;
        res = ak_dot(9, &x, inc, &mut y, inc, false);
        assert_eq!(res, 18);

        inc = 2;
        res = ak_dot(5, &x, inc, &mut y, inc, false);
        assert_eq!(res, 10);
    }

    #[test]
    fn ak_iamax_basic() {
        let x: Vec<i32> = vec![2, 5, 7, 3, 8, 4, 8, 1, 5];
        let mut inc: usize;
        let mut idx: usize;

        inc = 1;
        idx = ak_iamax(9, &x, inc);
        assert_eq!(idx, 4);

        inc = 2;
        idx = ak_iamax(5, &x, inc);
        assert_eq!(idx, 2);
    }

    #[test]
    fn ak_scal_basic() {
        let mut x: Vec<i32>;
        let mut inc: usize;

        x = vec![1; 9];
        inc = 1;
        ak_scal(9, 2, &mut x, inc);
        assert_eq!(x, vec![2; 9]);

        x = vec![1; 9];
        inc = 2;
        ak_scal(5, 2, &mut x, inc);
        assert_eq!(x, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]);
    }
}
