use std::ops::{Add, Mul, Neg};

// ***************************************************************************
// ***************************************************************************
// ROT trait and generic implementation
// ***************************************************************************
// ***************************************************************************

pub trait ROT<A, B>
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// rot (apply a Givens rotation) _
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 2)
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    /// * `y` - a vector
    /// * `inc_y` - increment between elements of `y`
    /// * `c` - cosine factor
    /// * `s` - sine factor
    fn mk_rot_2nsel(
        n: usize,
        x: &mut [Self],
        inc_x: usize,
        y: &mut [Self],
        inc_y: usize,
        c: A,
        s: B,
    ) -> ();

    /// mk (micro-kernel) _
    /// rot (apply a Givens rotation) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    /// * `c` - cosine factor
    /// * `s` - sine factor
    fn mk_rot_8sel(n: usize, x: &mut [Self], y: &mut [Self], c: A, s: B) -> ();
}

impl<T, A, B> ROT<A, B> for T
where
    T: Add<Output = T> + Copy,
    A: Copy + Mul<T, Output = T>,
    B: Copy + Mul<T, Output = T> + Neg<Output = B>,
{
    default fn mk_rot_2nsel(
        n: usize,
        x: &mut [Self],
        inc_x: usize,
        y: &mut [Self],
        inc_y: usize,
        c: A,
        s: B,
    ) -> () {
        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;
        let mut y2: usize = n / 2 * inc_y;

        for _ in 0..(n / 2) {
            let x_x1: T = x[x1];
            let y_y1: T = y[y1];
            let x_x2: T = x[x2];
            let y_y2: T = y[y2];

            x[x1] = c * x_x1 + s * y_y1;
            y[y1] = -s * x_x1 + c * y_y1;
            x[x2] = c * x_x2 + s * y_y2;
            y[y2] = -s * x_x2 + c * y_y2;

            x1 += inc_x;
            x2 += inc_x;
            y1 += inc_y;
            y2 += inc_y;
        }
    }

    default fn mk_rot_8sel(n: usize, x: &mut [Self], y: &mut [Self], c: A, s: B) -> () {
        for i in (0..n).step_by(8) {
            let x0 = x[i];
            let y0 = y[i];
            let x1 = x[i + 1];
            let y1 = y[i + 1];
            let x2 = x[i + 2];
            let y2 = y[i + 2];
            let x3 = x[i + 3];
            let y3 = y[i + 3];
            let x4 = x[i + 4];
            let y4 = y[i + 4];
            let x5 = x[i + 5];
            let y5 = y[i + 5];
            let x6 = x[i + 6];
            let y6 = y[i + 6];
            let x7 = x[i + 7];
            let y7 = y[i + 7];

            x[i] = c * x0 + s * y0;
            y[i] = -s * x0 + c * y0;
            x[i + 1] = c * x1 + s * y1;
            y[i + 1] = -s * x1 + c * y1;
            x[i + 2] = c * x2 + s * y2;
            y[i + 2] = -s * x2 + c * y2;
            x[i + 3] = c * x3 + s * y3;
            y[i + 3] = -s * x3 + c * y3;
            x[i + 4] = c * x4 + s * y4;
            y[i + 4] = -s * x4 + c * y4;
            x[i + 5] = c * x5 + s * y5;
            y[i + 5] = -s * x5 + c * y5;
            x[i + 6] = c * x6 + s * y6;
            y[i + 6] = -s * x6 + c * y6;
            x[i + 7] = c * x7 + s * y7;
            y[i + 7] = -s * x7 + c * y7;
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
    fn mk_rot_2nsel_basic() {
        let mut x: Vec<i32>;
        let mut y: Vec<i32>;
        let mut inc: usize;

        x = vec![1, 2, 3, 4, 5, 6, 7, 8];
        y = vec![9, 10, 11, 12];
        inc = 1;
        i32::mk_rot_2nsel(4, &mut x, inc, &mut y, inc, 0, 1);
        assert_eq!(x, vec![9, 10, 11, 12, 5, 6, 7, 8]);
        assert_eq!(y, vec![-1, -2, -3, -4]);

        x = vec![1, 2, 3, 4, 5, 6, 7, 8];
        y = vec![9, 10, 11, 12, 13, 14, 15, 16];
        inc = 2;
        i32::mk_rot_2nsel(4, &mut x, inc, &mut y, inc, 0, 1);
        assert_eq!(x, vec![9, 2, 11, 4, 13, 6, 15, 8]);
        assert_eq!(y, vec![-1, 10, -3, 12, -5, 14, -7, 16]);
    }

    #[test]
    fn mk_rot_8sel_basic() {
        let mut x: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut y: Vec<i32> = vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];

        i32::mk_rot_8sel(16, &mut x, &mut y, 0, 1);
        assert_eq!(
            x,
            vec![
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
            ]
        );
        assert_eq!(
            y,
            vec![
                -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16
            ]
        );
    }
}
