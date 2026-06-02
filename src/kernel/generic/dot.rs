use std::ops::{Add, AddAssign, Mul};

use crate::utils::traits::{Conjigatable, Zeroable};

// ***************************************************************************
// ***************************************************************************
// DOT trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait DOT<A>
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
    fn mk_dot_2nsel(n: usize, x: &[Self], inc_x: usize, y: &[Self], inc_y: usize) -> A;

    /// mk (micro-kernel) _
    /// dot (x^T * y) _
    /// conj (conjigate x before calculations)
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 2)
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    /// * `y` - a vector
    /// * `inc_y` - increment between elements of `y`
    fn mk_dot_conj_2nsel(n: usize, x: &[Self], inc_x: usize, y: &[Self], inc_y: usize) -> A;

    /// mk (micro-kernel) _
    /// dot (x^T * y) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    fn mk_dot_8sel(n: usize, x: &[Self], y: &[Self]) -> A;

    /// mk (micro-kernel) _
    /// dot (x^T * y) _
    /// conj (conjigate x before calculations)
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    fn mk_dot_conj_8sel(n: usize, x: &[Self], y: &[Self]) -> A;
}

impl<T, A> DOT<A> for T
where
    T: Add<Output = T> + Conjigatable + Copy + Mul<Output = T>,
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

    use crate::utils::types::{cf32, cf64};

    #[test]
    fn mk_dot_2nsel_basic() {
        let cx: Vec<cf32> = vec![
            cf32::new(1.0, 1.0),
            cf32::new(2.0, 1.0),
            cf32::new(3.0, 1.0),
            cf32::new(4.0, 1.0),
        ];
        let cy: Vec<cf32> = vec![cf32::new(2.0, 1.0); 4];

        let inc: usize = 1;

        let res: cf32 = cf32::mk_dot_2nsel(4, &cx, inc, &cy, inc);
        assert_eq!(res.real(), 16.0);
        assert_eq!(res.imag(), 18.0);

        let res: cf32 = cf32::mk_dot_conj_2nsel(4, &cx, inc, &cy, inc);
        assert_eq!(res.real(), 24.0);
        assert_eq!(res.imag(), 2.0);

        let res: cf64 = cf32::mk_dot_2nsel(4, &cx, inc, &cy, inc);
        assert_eq!(res.real(), 16.0);
        assert_eq!(res.imag(), 18.0);

        let res: cf64 = cf32::mk_dot_conj_2nsel(4, &cx, inc, &cy, inc);
        assert_eq!(res.real(), 24.0);
        assert_eq!(res.imag(), 2.0);
    }

    #[test]
    fn mk_dot_2nsel_stride_2() {
        let x: Vec<cf32> = vec![
            cf32::new(1.0, 1.0),
            cf32::new(99.0, 1.0),
            cf32::new(2.0, 1.0),
            cf32::new(99.0, 1.0),
            cf32::new(3.0, 1.0),
            cf32::new(99.0, 1.0),
            cf32::new(4.0, 1.0),
            cf32::new(99.0, 1.0),
        ];
        let y: Vec<cf32> = vec![cf32::new(2.0, 1.0); 8];

        let inc: usize = 2;

        let res: cf32 = cf32::mk_dot_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res.real(), 16.0);
        assert_eq!(res.imag(), 18.0);

        let res: cf32 = cf32::mk_dot_conj_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res.real(), 24.0);
        assert_eq!(res.imag(), 2.0);

        let res: cf64 = cf32::mk_dot_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res.real(), 16.0);
        assert_eq!(res.imag(), 18.0);

        let res: cf64 = cf32::mk_dot_conj_2nsel(4, &x, inc, &y, inc);
        assert_eq!(res.real(), 24.0);
        assert_eq!(res.imag(), 2.0);
    }

    #[test]
    fn mk_dot_8sel_basic() {
        let x: Vec<cf32> = vec![cf32::new(3.0, 1.0); 8];
        let y: Vec<cf32> = vec![cf32::new(2.0, 1.0); 8];

        let res: cf32 = cf32::mk_dot_8sel(4, &x, &y);
        assert_eq!(res.real(), 40.0);
        assert_eq!(res.imag(), 40.0);

        let res: cf32 = cf32::mk_dot_conj_8sel(4, &x, &y);
        assert_eq!(res.real(), 56.0);
        assert_eq!(res.imag(), 8.0);

        let res: cf64 = cf32::mk_dot_8sel(4, &x, &y);
        assert_eq!(res.real(), 40.0);
        assert_eq!(res.imag(), 40.0);

        let res: cf64 = cf32::mk_dot_conj_8sel(4, &x, &y);
        assert_eq!(res.real(), 56.0);
        assert_eq!(res.imag(), 8.0);
    }
}
