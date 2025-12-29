use std::{
    arch::x86_64::*,
    ops::{AddAssign, Mul},
};

use crate::{
    kernel::generic,
    utils::{mem::is_aligned, types::Float32},
};

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
    ///
    /// * `b` - Number of avx blocks in vectors `x` and `y`
    ///
    /// * `alpha` - scaling factor for `x`.
    ///
    /// * `x` - A vector.
    ///
    /// * `y` - A vector.
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

impl AXPY for Float32 {
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
        let x: Vec<usize> = vec![1; b * 32 / size_of::<usize>()];
        let mut y: Vec<usize> = vec![1; b * 32 / size_of::<usize>()];
        unsafe { usize::mk_axpy_bssel_avx(b, 2, &x, &mut y) };
        assert_eq!(y, vec![3; b * 32 / size_of::<usize>()]);
    }

    #[test]
    fn mk_axpy_bssel_avx_float32() {
        let b: usize = 2;
        let x: Vec<Float32> = vec![1.0; b * 32 / size_of::<Float32>()];
        let mut y: Vec<Float32> = vec![1.0; b * 32 / size_of::<Float32>()];
        unsafe { Float32::mk_axpy_bssel_avx(b, 2.0, &x, &mut y) };
        assert_eq!(y, vec![3.0; b * 32 / size_of::<Float32>()]);
    }

    #[test]
    #[should_panic]
    fn mk_axpy_bssel_avx_invalid_bounds() {
        let b: usize = 2;
        let x: Vec<usize> = vec![1; b * 32 / size_of::<usize>()];
        let mut y: Vec<usize> = vec![1; b * 32 / size_of::<usize>()];
        unsafe { usize::mk_axpy_bssel_avx(3, 2, &x, &mut y) };
    }
}
