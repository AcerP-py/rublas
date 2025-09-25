use std::arch::x86_64::*;

use crate::utils::types::SFLOAT;

/// mk (micro-kernel) _
/// sdot (SFLOAT dot product) _
/// s8el (sequential 8 element) _
/// avx (avx enabled)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 8 and within bounds).
///
/// * `x` - A vector.
///
/// * `y` - A vector.
///
/// # Returns
///
/// The dot product of `x` and `y`.
#[target_feature(enable = "avx")]
pub unsafe fn mk_sdot_s8el_avx(n: usize, x: &[SFLOAT], y: &[SFLOAT]) -> SFLOAT {
    let mut result_m256: __m256 = _mm256_setzero_ps();

    for i in (0..n).step_by(8) {
        unsafe {
            let xi: __m256 = _mm256_loadu_ps(x.as_ptr().add(i));
            let yi: __m256 = _mm256_loadu_ps(y.as_ptr().add(i));
            let prod: __m256 = _mm256_mul_ps(xi, yi);
            result_m256 = _mm256_add_ps(prod, result_m256);
        }
    }

    let mut result_array: [SFLOAT; 8] = [0.0; 8];
    unsafe {
        _mm256_storeu_ps(result_array.as_mut_ptr(), result_m256);
    }

    result_array[0]
        + result_array[1]
        + result_array[2]
        + result_array[3]
        + result_array[4]
        + result_array[5]
        + result_array[6]
        + result_array[7]
}
