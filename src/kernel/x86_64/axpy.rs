use std::{
    arch::x86_64::*,
    ops::{AddAssign, Mul},
};

use crate::{kernel::generic, utils::mem::is_aligned};

use super::utils::BlockSizes;

// ***************************************************************************
// ***************************************************************************
// AXPY trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait AXPY: generic::axpy::AXPY + BlockSizes
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// axpy (y = y + ax) _
    /// bssel (consumes elements in blocks of sequential elements) _
    /// avx (avx accelerated)
    ///
    /// # Arguments
    /// * `b` - number of avx blocks in vectors `x` and `y`
    /// * `alpha` - scaling factor for `x`
    /// * `x` - a vector
    /// * `y` - a vector
    unsafe fn mk_axpy_bssel_avx(b: usize, alpha: Self, x: &[Self], y: &mut [Self]) -> ();
}

impl<T> AXPY for T
where
    Self: AddAssign + Copy + Mul<Output = Self> + BlockSizes,
{
    #[target_feature(enable = "avx")]
    default unsafe fn mk_axpy_bssel_avx(b: usize, alpha: Self, x: &[Self], y: &mut [Self]) -> () {
        for i in 0..(b * T::AVX_BLOCK_SIZE) {
            y[i] += alpha * x[i]
        }
    }
}

// ***************************************************************************
// ***************************************************************************
// specific kernels
// ***************************************************************************
// ***************************************************************************

impl AXPY for f32 {
    #[target_feature(enable = "avx")]
    unsafe fn mk_axpy_bssel_avx(b: usize, alpha: Self, x: &[Self], y: &mut [Self]) -> () {
        let x_ptr: *const Self = x.as_ptr();
        let y_ptr: *mut Self = y.as_mut_ptr();

        let a_vec: __m256 = _mm256_set1_ps(alpha);
        let mut r_vec: __m256;

        if is_aligned(y, 32) && is_aligned(x, 32) {
            for i in (0..(b * Self::AVX_BLOCK_SIZE)).step_by(8) {
                unsafe {
                    let x_vec: __m256 = _mm256_load_ps(x_ptr.add(i));
                    let y_vec: __m256 = _mm256_load_ps(y_ptr.add(i));
                    r_vec = _mm256_mul_ps(a_vec, x_vec);
                    r_vec = _mm256_add_ps(y_vec, r_vec);
                    _mm256_storeu_ps(y_ptr.add(i), r_vec);
                }
            }
        } else {
            for i in (0..(b * Self::AVX_BLOCK_SIZE)).step_by(8) {
                unsafe {
                    let x_vec: __m256 = _mm256_loadu_ps(x_ptr.add(i));
                    let y_vec: __m256 = _mm256_loadu_ps(y_ptr.add(i));
                    r_vec = _mm256_mul_ps(a_vec, x_vec);
                    r_vec = _mm256_add_ps(y_vec, r_vec);
                    _mm256_storeu_ps(y_ptr.add(i), r_vec);
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
    fn mk_axpy_bssel_avx_basic() {
        let b: usize = 2;
        let x: Vec<i32> = vec![1; b * i32::AVX_BLOCK_SIZE];
        let mut y: Vec<i32>;

        y = vec![2; b * i32::AVX_BLOCK_SIZE];
        unsafe { i32::mk_axpy_bssel_avx(b, 2, &x, &mut y) };
        assert_eq!(y, vec![4; b * i32::AVX_BLOCK_SIZE]);
    }

    #[test]
    fn mk_axpy_bssel_avx_f32() {
        let b: usize = 2;
        let x: Vec<f32> = vec![1.0; b * f32::AVX_BLOCK_SIZE];
        let mut y: Vec<f32>;

        y = vec![2.0; b * f32::AVX_BLOCK_SIZE];
        unsafe { f32::mk_axpy_bssel_avx(b, 2.0, &x, &mut y) };
        assert_eq!(y, vec![4.0; b * f32::AVX_BLOCK_SIZE]);
    }
}
