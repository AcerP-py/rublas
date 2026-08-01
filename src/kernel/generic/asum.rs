use crate::utils::traits::{BLASAbsoluteValue, Zeroable};
use std::ops::AddAssign;

// ***************************************************************************
// ***************************************************************************
// Asum trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait Asum<A>
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// asum (sum of "absolute" (|Re(x)|+|Im(x)|) values) _
    /// nsel (consumes non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vector `x`
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    ///
    /// # Returns
    /// the sum of absolute values of vector elements
    fn mk_asum_nsel(n: usize, x: &[Self], inc_x: usize) -> A;

    /// mk (micro-kernel) _
    /// asum (sum of "absolute" (|Re(x)|+|Im(x)|) values) _
    /// sel (consumes sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vector `x`
    /// * `x` - a vector
    ///
    /// # Returns
    /// the sum of absolute values of vector elements
    fn mk_asum_sel(n: usize, x: &[Self]) -> A;
}

impl<T, A> Asum<A> for T
where
    T: BLASAbsoluteValue<Output = A> + Copy,
    A: AddAssign + Zeroable,
{
    default fn mk_asum_nsel(n: usize, x: &[Self], inc_x: usize) -> A {
        let mut result: A = A::zero();

        for i in 0..n {
            result += x[i * inc_x].blas_abs();
        }

        result
    }

    default fn mk_asum_sel(n: usize, x: &[Self]) -> A {
        let mut result: A = A::zero();

        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            result += x[i].blas_abs();
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

    #[test]
    fn mk_asum_nsel_basic() {
        let x: Vec<i32> = vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10];
        let mut inc: usize;
        let mut result: i32;

        inc = 1;
        result = i32::mk_asum_nsel(10, &x, inc);
        assert_eq!(result, 55);

        inc = 2;
        result = i32::mk_asum_nsel(5, &x, inc);
        assert_eq!(result, 25);
    }

    #[test]
    fn mk_asum_sel_basic() {
        let x: Vec<i32> = vec![-1, 2, -3, 4, -5, 6, -7, 8];

        let result: i32 = i32::mk_asum_sel(8, &x);
        assert_eq!(result, 36);
    }
}
