use crate::utils::traits::BLASAbsoluteValue;

// ***************************************************************************
// ***************************************************************************
// Iamax trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait Iamax
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// iamax (the element with the largest "absolute" (|Re(x)|+|Im(x)|) value) _
    /// nsel (consumes non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vector `x
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    ///
    /// # Returns
    /// the index of the first element with the largest absolute value
    fn mk_iamax_nsel(n: usize, x: &[Self], inc_x: usize) -> usize;

    /// mk (micro-kernel) _
    /// iamax (the element with the largest "absolute" (|Re(x)|+|Im(x)|) value) _
    /// nsel (consumes sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vector `x`
    /// * `x` - a vector
    ///
    /// # Returns
    /// the index of the first element with the largest absolute value
    fn mk_iamax_sel(n: usize, x: &[Self]) -> usize;
}

impl<T> Iamax for T
where
    T: BLASAbsoluteValue + Copy,
{
    default fn mk_iamax_nsel(n: usize, x: &[Self], inc_x: usize) -> usize {
        let mut max: T::Output = x[0].blas_abs();
        let mut idx: usize = 0;

        for i in 1..n {
            if x[i * inc_x].blas_abs() > max {
                max = x[i * inc_x].blas_abs();
                idx = i;
            }
        }

        idx
    }

    default fn mk_iamax_sel(n: usize, x: &[Self]) -> usize {
        let mut max: T::Output = x[0].blas_abs();
        let mut idx: usize = 0;

        #[allow(clippy::needless_range_loop)]
        for i in 1..n {
            if x[i].blas_abs() > max {
                max = x[i].blas_abs();
                idx = i;
            }
        }

        idx
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
    fn mk_iamax_nsel_basic() {
        let x: Vec<i32> = vec![4, 7, 1, 8, 3, 0, 2, 9, 4, 3];
        let mut inc: usize;
        let mut idx: usize;

        inc = 1;
        idx = i32::mk_iamax_nsel(10, &x, inc);
        assert_eq!(idx, 7);

        inc = 2;
        idx = i32::mk_iamax_nsel(5, &x, inc);
        assert_eq!(idx, 0);
    }

    #[test]
    fn mk_iamax_sel_basic() {
        let x: Vec<i32> = vec![4, 7, 1, 8, 3, 0, 2, 9, 4, 3];

        let idx: usize = i32::mk_iamax_sel(10, &x);
        assert_eq!(idx, 7);
    }
}
