use crate::utils::enums::RotmFlag;
use std::ops::{Add, Mul};

// ***************************************************************************
// ***************************************************************************
// Rotm trait and generic implementation
// ***************************************************************************
// ***************************************************************************

pub trait Rotm
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// rotm (apply a modified Givens rotation) _
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 2)
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    /// * `y` - a vector
    /// * `inc_y` - increment between elements of `y`
    /// * `flag` - type of modified rotation to apply
    /// * `params` - 4 element (2x2) transformation matrix
    fn mk_rotm_2nsel(
        n: usize,
        x: &mut [Self],
        inc_x: usize,
        y: &mut [Self],
        inc_y: usize,
        flag: RotmFlag,
        params: &[Self],
    );

    /// mk (micro-kernel) _
    /// rotm (apply a modified Givens rotation) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    /// * `flag` - type of modified rotation to apply
    /// * `params` - 4 element (2x2) transformation matrix
    fn mk_rotm_8sel(n: usize, x: &mut [Self], y: &mut [Self], flag: RotmFlag, params: &[Self]);
}

impl<T> Rotm for T
where
    T: Add<Output = T> + Copy + Mul<Output = T>,
{
    default fn mk_rotm_2nsel(
        n: usize,
        x: &mut [T],
        inc_x: usize,
        y: &mut [T],
        inc_y: usize,
        flag: RotmFlag,
        params: &[T],
    ) {
        let p11: T = params[0];
        let p21: T = params[1];
        let p12: T = params[2];
        let p22: T = params[3];

        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;
        let mut y2: usize = n / 2 * inc_y;

        for _ in 0..(n / 2) {
            let x_x1: T = x[x1];
            let y_y1: T = y[y1];
            let x_x2: T = x[x2];
            let y_y2: T = y[y2];

            match flag {
                RotmFlag::General => {
                    x[x1] = p11 * x_x1 + p12 * y_y1;
                    y[y1] = p21 * x_x1 + p22 * y_y1;
                    x[x2] = p11 * x_x2 + p12 * y_y2;
                    y[y2] = p21 * x_x2 + p22 * y_y2;
                }
                RotmFlag::Diagonal => {
                    x[x1] = p11 * x_x1;
                    y[y1] = p22 * y_y1;
                    x[x2] = p11 * x_x2;
                    y[y2] = p22 * y_y2;
                }
                RotmFlag::Shear => {
                    x[x1] = x_x1 + p12 * y_y1;
                    y[y1] = p21 * x_x1 + y_y1;
                    x[x2] = x_x2 + p12 * y_y2;
                    y[y2] = p21 * x_x2 + y_y2;
                }
                RotmFlag::Identity => {
                    // x and y remain unchanged
                }
            }

            x1 += inc_x;
            x2 += inc_x;
            y1 += inc_y;
            y2 += inc_y;
        }
    }

    default fn mk_rotm_8sel(n: usize, x: &mut [T], y: &mut [T], flag: RotmFlag, params: &[T]) {
        let p11: T = params[0];
        let p21: T = params[1];
        let p12: T = params[2];
        let p22: T = params[3];

        for i in (0..n).step_by(8) {
            let x0: T = x[i];
            let y0: T = y[i];
            let x1: T = x[i + 1];
            let y1: T = y[i + 1];
            let x2: T = x[i + 2];
            let y2: T = y[i + 2];
            let x3: T = x[i + 3];
            let y3: T = y[i + 3];
            let x4: T = x[i + 4];
            let y4: T = y[i + 4];
            let x5: T = x[i + 5];
            let y5: T = y[i + 5];
            let x6: T = x[i + 6];
            let y6: T = y[i + 6];
            let x7: T = x[i + 7];
            let y7: T = y[i + 7];

            match flag {
                RotmFlag::General => {
                    x[i] = p11 * x0 + p12 * y0;
                    y[i] = p21 * x0 + p22 * y0;
                    x[i + 1] = p11 * x1 + p12 * y1;
                    y[i + 1] = p21 * x1 + p22 * y1;
                    x[i + 2] = p11 * x2 + p12 * y2;
                    y[i + 2] = p21 * x2 + p22 * y2;
                    x[i + 3] = p11 * x3 + p12 * y3;
                    y[i + 3] = p21 * x3 + p22 * y3;
                    x[i + 4] = p11 * x4 + p12 * y4;
                    y[i + 4] = p21 * x4 + p22 * y4;
                    x[i + 5] = p11 * x5 + p12 * y5;
                    y[i + 5] = p21 * x5 + p22 * y5;
                    x[i + 6] = p11 * x6 + p12 * y6;
                    y[i + 6] = p21 * x6 + p22 * y6;
                    x[i + 7] = p11 * x7 + p12 * y7;
                    y[i + 7] = p21 * x7 + p22 * y7;
                }
                RotmFlag::Diagonal => {
                    x[i] = p11 * x0;
                    y[i] = p22 * y0;
                    x[i + 1] = p11 * x1;
                    y[i + 1] = p22 * y1;
                    x[i + 2] = p11 * x2;
                    y[i + 2] = p22 * y2;
                    x[i + 3] = p11 * x3;
                    y[i + 3] = p22 * y3;
                    x[i + 4] = p11 * x4;
                    y[i + 4] = p22 * y4;
                    x[i + 5] = p11 * x5;
                    y[i + 5] = p22 * y5;
                    x[i + 6] = p11 * x6;
                    y[i + 6] = p22 * y6;
                    x[i + 7] = p11 * x7;
                    y[i + 7] = p22 * y7;
                }
                RotmFlag::Shear => {
                    x[i] = x0 + p12 * y0;
                    y[i] = p21 * x0 + y0;
                    x[i + 1] = x1 + p12 * y1;
                    y[i + 1] = p21 * x1 + y1;
                    x[i + 2] = x2 + p12 * y2;
                    y[i + 2] = p21 * x2 + y2;
                    x[i + 3] = x3 + p12 * y3;
                    y[i + 3] = p21 * x3 + y3;
                    x[i + 4] = x4 + p12 * y4;
                    y[i + 4] = p21 * x4 + y4;
                    x[i + 5] = x5 + p12 * y5;
                    y[i + 5] = p21 * x5 + y5;
                    x[i + 6] = x6 + p12 * y6;
                    y[i + 6] = p21 * x6 + y6;
                    x[i + 7] = x7 + p12 * y7;
                    y[i + 7] = p21 * x7 + y7;
                }
                RotmFlag::Identity => {
                    // x and y remain unchanged
                }
            }
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
    fn mk_rotm_2nsel_basic() {
        let mut x: Vec<i32>;
        let mut y: Vec<i32>;
        let mut inc: usize;
        let param: Vec<i32> = vec![-1, -1, -1, -1];

        // general
        x = vec![1, 2, 3, 4, 5, 6, 7, 8];
        y = vec![9, 10, 11, 12];
        inc = 1;
        i32::mk_rotm_2nsel(4, &mut x, inc, &mut y, inc, RotmFlag::General, &param);
        assert_eq!(x, vec![-10, -12, -14, -16, 5, 6, 7, 8]);
        assert_eq!(y, vec![-10, -12, -14, -16]);

        x = vec![1, 2, 3, 4, 5, 6, 7, 8];
        y = vec![9, 10, 11, 12, 13, 14, 15, 16];
        inc = 2;
        i32::mk_rotm_2nsel(4, &mut x, inc, &mut y, inc, RotmFlag::General, &param);
        assert_eq!(x, vec![-10, 2, -14, 4, -18, 6, -22, 8]);
        assert_eq!(y, vec![-10, 10, -14, 12, -18, 14, -22, 16]);

        // diagonal
        x = vec![1, 2, 3, 4];
        y = vec![5, 6, 7, 8];
        inc = 1;
        i32::mk_rotm_2nsel(4, &mut x, inc, &mut y, inc, RotmFlag::Diagonal, &param);
        assert_eq!(x, vec![-1, -2, -3, -4]);
        assert_eq!(y, vec![-5, -6, -7, -8]);

        // shear
        x = vec![1, 2, 3, 4];
        y = vec![5, 6, 7, 8];
        i32::mk_rotm_2nsel(4, &mut x, inc, &mut y, inc, RotmFlag::Shear, &param);
        assert_eq!(x, vec![-4, -4, -4, -4]);
        assert_eq!(y, vec![4, 4, 4, 4]);

        // identity
        x = vec![1, 2, 3, 4];
        y = vec![5, 6, 7, 8];
        i32::mk_rotm_2nsel(4, &mut x, inc, &mut y, inc, RotmFlag::Identity, &param);
        assert_eq!(x, vec![1, 2, 3, 4]);
        assert_eq!(y, vec![5, 6, 7, 8]);
    }

    #[test]
    fn mk_rotm_8sel_basic() {
        let mut x: Vec<i32>;
        let mut y: Vec<i32>;
        let param: Vec<i32> = vec![-1, -1, -1, -1];

        // general
        x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        y = vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        i32::mk_rotm_8sel(16, &mut x, &mut y, RotmFlag::General, &param);
        assert_eq!(
            x,
            vec![
                -18, -20, -22, -24, -26, -28, -30, -32, -34, -36, -38, -40, -42, -44, -46, -48
            ]
        );
        assert_eq!(
            y,
            vec![
                -18, -20, -22, -24, -26, -28, -30, -32, -34, -36, -38, -40, -42, -44, -46, -48
            ]
        );

        // diagonal
        x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        y = vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        i32::mk_rotm_8sel(16, &mut x, &mut y, RotmFlag::Diagonal, &param);
        assert_eq!(
            x,
            vec![
                -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16
            ]
        );
        assert_eq!(
            y,
            vec![
                -17, -18, -19, -20, -21, -22, -23, -24, -25, -26, -27, -28, -29, -30, -31, -32
            ]
        );

        // shear
        x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        y = vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        i32::mk_rotm_8sel(16, &mut x, &mut y, RotmFlag::Shear, &param);
        assert_eq!(
            x,
            vec![
                -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16
            ]
        );
        assert_eq!(
            y,
            vec![
                16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16
            ]
        );

        // identity
        x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        y = vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        i32::mk_rotm_8sel(16, &mut x, &mut y, RotmFlag::Identity, &param);
        assert_eq!(
            x,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        );
        assert_eq!(
            y,
            vec![
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
            ]
        );
    }
}
