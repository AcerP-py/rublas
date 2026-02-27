use std::ops::{AddAssign, Mul};

// ***************************************************************************
// ***************************************************************************
// AXPY trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait AXPY
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// axpy (y = y + ax) _
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    ///
    /// * `n` - length of vectors `x` and `y`
    /// (must be divisible by 2)
    ///
    /// * `alpha` - scaling factor for `x`
    ///
    /// * `x` - a vector
    ///
    /// * `inc_x` - increment between elements of `x`
    ///
    /// * `y` - a vector
    ///
    /// * `inc_y` - increment between elements of `y`
    fn mk_axpy_2nsel(
        n: usize,
        alpha: Self,
        x: &[Self],
        inc_x: usize,
        y: &mut [Self],
        inc_y: usize,
    ) -> ();

    /// mk (micro-kernel) _
    /// axpy (y = y + ax) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    ///
    /// * `n` - length of vectors `x` and `y`
    /// (must be divisible by 8)
    ///
    /// * `alpha` - scaling factor for `x`
    ///
    /// * `x` - a vector
    ///
    /// * `y` - a vector
    fn mk_axpy_8sel(n: usize, alpha: Self, x: &[Self], y: &mut [Self]) -> ();
}

impl<T> AXPY for T
where
    Self: AddAssign + Copy + Mul<Output = Self>,
{
    default fn mk_axpy_2nsel(
        n: usize,
        alpha: Self,
        x: &[Self],
        inc_x: usize,
        y: &mut [Self],
        inc_y: usize,
    ) -> () {
        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;
        let mut y2: usize = n / 2 * inc_y;

        for _ in 0..(n / 2) {
            y[y1] += alpha * x[x1];
            y[y2] += alpha * x[x2];
            x1 += inc_x;
            x2 += inc_x;
            y1 += inc_y;
            y2 += inc_y;
        }
    }

    default fn mk_axpy_8sel(n: usize, alpha: Self, x: &[Self], y: &mut [Self]) -> () {
        for i in (0..n).step_by(8) {
            y[i] += alpha * x[i];
            y[i + 1] += alpha * x[i + 1];
            y[i + 2] += alpha * x[i + 2];
            y[i + 3] += alpha * x[i + 3];
            y[i + 4] += alpha * x[i + 4];
            y[i + 5] += alpha * x[i + 5];
            y[i + 6] += alpha * x[i + 6];
            y[i + 7] += alpha * x[i + 7];
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
    fn mk_axpy_2nsel_basic() {
        let x: Vec<usize> = vec![1, 2, 3, 4];
        let mut y: Vec<usize> = vec![1, 2, 3, 4];
        let inc: usize = 1;
        usize::mk_axpy_2nsel(4, 2, &x, inc, &mut y, inc);
        assert_eq!(y, vec![3, 6, 9, 12]);
    }

    #[test]
    fn mk_axpy_2nsel_stride_2() {
        let x: Vec<usize> = vec![1, 99, 2, 99, 3, 99, 4, 99];
        let mut y: Vec<usize> = vec![1, 99, 2, 99, 3, 99, 4, 99];
        let inc: usize = 2;
        usize::mk_axpy_2nsel(4, 2, &x, inc, &mut y, inc);
        assert_eq!(y, vec![3, 99, 6, 99, 9, 99, 12, 99]);
    }

    #[test]
    fn mk_axpy_8sel_basic() {
        let x: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut y: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        usize::mk_axpy_8sel(8, 2, &x, &mut y);
        assert_eq!(y, vec![3, 6, 9, 12, 15, 18, 21, 24]);
    }
}
