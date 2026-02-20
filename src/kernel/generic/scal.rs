use std::ops::Mul;

// ***************************************************************************
// ***************************************************************************
// SCAL trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait SCAL
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// scal (x = ax) _
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    ///
    /// * `n` - Length of vector `x
    /// (must be divisible by 2 and within bounds).
    ///
    /// * `alpha` - scaling factor for `x`.
    ///
    /// * `x` - A vector.
    ///
    /// * `inc_x` - Increment between elements of `x`.
    fn mk_scal_2nsel(n: usize, alpha: Self, x: &mut [Self], inc_x: usize) -> ();

    /// mk (micro-kernel) _
    /// scal (x = ax) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    ///
    /// * `n` - Length of vector `x`
    /// (must be divisible by 8 and within bounds).
    ///
    /// * `alpha` - scaling factor for `x`.
    ///
    /// * `x` - A vector.
    fn mk_scal_8sel(n: usize, alpha: Self, x: &mut [Self]) -> ();
}

impl<T> SCAL for T
where
    Self: Copy + Mul<Output = Self>,
{
    default fn mk_scal_2nsel(n: usize, alpha: Self, x: &mut [Self], inc_x: usize) -> () {
        let mut x1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;

        for _ in 0..(n / 2) {
            x[x1] = alpha * x[x1];
            x[x2] = alpha * x[x2];
            x1 += inc_x;
            x2 += inc_x;
        }
    }

    default fn mk_scal_8sel(n: usize, alpha: Self, x: &mut [Self]) -> () {
        for i in (0..n).step_by(8) {
            x[i] = alpha * x[i];
            x[i + 1] = alpha * x[i + 1];
            x[i + 2] = alpha * x[i + 2];
            x[i + 3] = alpha * x[i + 3];
            x[i + 4] = alpha * x[i + 4];
            x[i + 5] = alpha * x[i + 5];
            x[i + 6] = alpha * x[i + 6];
            x[i + 7] = alpha * x[i + 7];
        }
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
    fn mk_scal_2nsel_basic() {
        let mut x: Vec<usize> = vec![1, 2, 3, 4];
        let inc: usize = 1;
        usize::mk_scal_2nsel(4, 2, &mut x, inc);
        assert_eq!(x, vec![2, 4, 6, 8]);
    }

    #[test]
    fn mk_scal_2nsel_stride_2() {
        let mut x: Vec<usize> = vec![1, 99, 2, 99, 3, 99, 4, 99];
        let inc: usize = 2;
        usize::mk_scal_2nsel(4, 2, &mut x, inc);
        assert_eq!(x, vec![2, 99, 4, 99, 6, 99, 8, 99]);
    }

    #[test]
    fn mk_scal_8sel_basic() {
        let mut x: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        usize::mk_scal_8sel(8, 2, &mut x);
        assert_eq!(x, vec![2, 4, 6, 8, 10, 12, 14, 16]);
    }

    #[test]
    #[should_panic]
    fn mk_scal_2nsel_invalid_bounds() {
        let mut x: Vec<usize> = vec![1; 2];
        let inc: usize = 1;
        usize::mk_scal_2nsel(4, 2, &mut x, inc);
    }

    #[test]
    #[should_panic]
    fn mk_scal_8sel_invalid_bounds() {
        let mut x: Vec<usize> = vec![1; 4];
        usize::mk_scal_8sel(8, 2, &mut x);
    }
}
