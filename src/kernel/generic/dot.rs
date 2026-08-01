use crate::utils::traits::{Conjugatable, Zeroable};
use std::ops::{Add, AddAssign, Mul};

// ***************************************************************************
// ***************************************************************************
// Dot trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait Dot<A>
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// dot (x^T * y) _
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 2)
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    /// * `y` - a vector
    /// * `inc_y` - increment between elements of `y`
    ///
    /// # Returns
    /// the dot product of vectors 'x' and 'y'
    fn mk_dot_2nsel(n: usize, x: &[Self], inc_x: usize, y: &[Self], inc_y: usize) -> A;

    /// mk (micro-kernel) _
    /// dot (x^T * y) _
    /// conj (conjugate x before calculations)
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 2)
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    /// * `y` - a vector
    /// * `inc_y` - increment between elements of `y`
    ///
    /// # Returns
    /// the dot product of vectors 'conjugated x' and 'y'
    fn mk_dot_conj_2nsel(n: usize, x: &[Self], inc_x: usize, y: &[Self], inc_y: usize) -> A;

    /// mk (micro-kernel) _
    /// dot (x^T * y) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    ///
    /// # Returns
    /// the dot product of vectors 'x' and 'y'
    fn mk_dot_8sel(n: usize, x: &[Self], y: &[Self]) -> A;

    /// mk (micro-kernel) _
    /// dot (x^T * y) _
    /// conj (conjugate x before calculations)
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    ///
    /// # Returns
    /// the dot product of vectors 'conjugated x' and 'y'
    fn mk_dot_conj_8sel(n: usize, x: &[Self], y: &[Self]) -> A;
}

impl<T, A> Dot<A> for T
where
    T: Add<Output = T> + Conjugatable + Copy + Mul<Output = T>,
    A: AddAssign + From<T> + Zeroable,
{
    default fn mk_dot_2nsel(n: usize, x: &[Self], inc_x: usize, y: &[Self], inc_y: usize) -> A {
        let mut result: A = A::zero();

        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;
        let mut y2: usize = n / 2 * inc_y;

        for _ in 0..(n / 2) {
            result += A::from(x[x1] * y[y1] + x[x2] * y[y2]);
            x1 += inc_x;
            x2 += inc_x;
            y1 += inc_y;
            y2 += inc_y;
        }

        result
    }

    default fn mk_dot_conj_2nsel(
        n: usize,
        x: &[Self],
        inc_x: usize,
        y: &[Self],
        inc_y: usize,
    ) -> A {
        let mut result: A = A::zero();

        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;
        let mut y2: usize = n / 2 * inc_y;

        for _ in 0..(n / 2) {
            result += A::from(x[x1].conj() * y[y1] + x[x2].conj() * y[y2]);
            x1 += inc_x;
            x2 += inc_x;
            y1 += inc_y;
            y2 += inc_y;
        }

        result
    }

    default fn mk_dot_8sel(n: usize, x: &[Self], y: &[Self]) -> A {
        let mut result: A = A::zero();

        for i in (0..n).step_by(8) {
            result += A::from(
                x[i] * y[i]
                    + x[i + 1] * y[i + 1]
                    + x[i + 2] * y[i + 2]
                    + x[i + 3] * y[i + 3]
                    + x[i + 4] * y[i + 4]
                    + x[i + 5] * y[i + 5]
                    + x[i + 6] * y[i + 6]
                    + x[i + 7] * y[i + 7],
            );
        }

        result
    }

    default fn mk_dot_conj_8sel(n: usize, x: &[Self], y: &[Self]) -> A {
        let mut result: A = A::zero();

        for i in (0..n).step_by(8) {
            result += A::from(
                x[i].conj() * y[i]
                    + x[i + 1].conj() * y[i + 1]
                    + x[i + 2].conj() * y[i + 2]
                    + x[i + 3].conj() * y[i + 3]
                    + x[i + 4].conj() * y[i + 4]
                    + x[i + 5].conj() * y[i + 5]
                    + x[i + 6].conj() * y[i + 6]
                    + x[i + 7].conj() * y[i + 7],
            );
        }

        result
    }
}

// ***************************************************************************
// ***************************************************************************
// tests
// ***************************************************************************
// ***************************************************************************

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::complex::Complex;

    impl From<Complex<i32>> for Complex<i64> {
        fn from(value: Complex<i32>) -> Self { Self::new(value.real() as i64, value.imag() as i64) }
    }

    #[test]
    fn mk_dot_2nsel_basic() {
        let x: Vec<Complex<i32>> = vec![Complex::new(2, 1); 8];
        let y: Vec<Complex<i32>> = vec![
            Complex::new(1, 1),
            Complex::new(2, 1),
            Complex::new(3, 1),
            Complex::new(4, 1),
            Complex::new(5, 1),
            Complex::new(6, 1),
            Complex::new(7, 1),
            Complex::new(8, 1),
        ];
        let mut inc: usize;
        let mut res32: Complex<i32>;
        let mut res64: Complex<i64>;

        inc = 1;
        res32 = Complex::<i32>::mk_dot_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res32.real(), 16);
        assert_eq!(res32.imag(), 18);

        // test type up-casting
        res64 = Complex::<i32>::mk_dot_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res64.real(), 16);
        assert_eq!(res64.imag(), 18);

        inc = 2;
        res32 = Complex::<i32>::mk_dot_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res32.real(), 28);
        assert_eq!(res32.imag(), 24);

        // test type up-casting
        res64 = Complex::<i32>::mk_dot_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res64.real(), 28);
        assert_eq!(res64.imag(), 24);
    }

    #[test]
    fn mk_dot_2nsel_conj_basic() {
        let x: Vec<Complex<i32>> = vec![Complex::new(2, 1); 8];
        let y: Vec<Complex<i32>> = vec![
            Complex::new(1, 1),
            Complex::new(2, 1),
            Complex::new(3, 1),
            Complex::new(4, 1),
            Complex::new(5, 1),
            Complex::new(6, 1),
            Complex::new(7, 1),
            Complex::new(8, 1),
        ];
        let mut inc: usize;
        let mut res32: Complex<i32>;
        let mut res64: Complex<i64>;

        inc = 1;
        res32 = Complex::<i32>::mk_dot_conj_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res32.real(), 24);
        assert_eq!(res32.imag(), -2);

        // test type up-casting
        res64 = Complex::<i32>::mk_dot_conj_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res64.real(), 24);
        assert_eq!(res64.imag(), -2);

        inc = 2;
        res32 = Complex::<i32>::mk_dot_conj_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res32.real(), 36);
        assert_eq!(res32.imag(), -8);

        // test type up-casting
        res64 = Complex::<i32>::mk_dot_conj_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res64.real(), 36);
        assert_eq!(res64.imag(), -8);
    }

    #[test]
    fn mk_dot_8sel_basic() {
        let x: Vec<Complex<i32>> = vec![Complex::new(3, 1); 16];
        let y: Vec<Complex<i32>> = vec![Complex::new(2, 1); 16];

        let res32: Complex<i32> = Complex::<i32>::mk_dot_8sel(16, &x, &y);
        assert_eq!(res32.real(), 80);
        assert_eq!(res32.imag(), 80);

        // test type up-casting
        let res64: Complex<i64> = Complex::<i32>::mk_dot_8sel(16, &x, &y);
        assert_eq!(res64.real(), 80);
        assert_eq!(res64.imag(), 80);
    }

    #[test]
    fn mk_dot_8sel_conj_basic() {
        let x: Vec<Complex<i32>> = vec![Complex::new(3, 1); 16];
        let y: Vec<Complex<i32>> = vec![Complex::new(2, 1); 16];

        let res32: Complex<i32> = Complex::<i32>::mk_dot_conj_8sel(16, &x, &y);
        assert_eq!(res32.real(), 112);
        assert_eq!(res32.imag(), 16);

        // test type up-casting
        let res64: Complex<i64> = Complex::<i32>::mk_dot_conj_8sel(16, &x, &y);
        assert_eq!(res64.real(), 112);
        assert_eq!(res64.imag(), 16);
    }
}
