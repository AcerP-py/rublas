use std::ops::{AddAssign, Mul};

pub mod utils;
pub mod axpy;

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
        if is_x86_feature_detected!("avx") && n >= T::AVX_BLOCK_SIZE {
            c = n & !(T::AVX_BLOCK_SIZE - 1);
            let b: usize = c / T::AVX_BLOCK_SIZE;
            unsafe { T::mk_axpy_bssel_avx(b, alpha, x, y) };
        } else {
            c = n & !7;
            T::mk_axpy_8sel(c, alpha, x, y);
        }
    } else {
        c = n & !1;
        T::mk_axpy_2nsel(c, alpha, x, inc_x, y, inc_y);
    }

    for i in c..n {
        y[i * inc_y] += alpha * x[i * inc_x];
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
}
