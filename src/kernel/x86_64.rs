use crate::kernel::generic::templates;
use crate::utils::types::SFLOAT;

// ===========================================================================
// ===========================================================================
// operation modules
// ===========================================================================
// ===========================================================================

// ===========================================================================
// dot
// ===========================================================================

mod sdot;

// ===========================================================================
// ===========================================================================
// architecture kernels
// ===========================================================================
// ===========================================================================

// ===========================================================================
// dot
// ===========================================================================

/// ak (architecture-kernel) _
/// sdot (SFLOAT dot product)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`.
///
/// * `x` - A vector.
///
/// * `inc_x` - Increment between elements of `x`.
///
/// * `y` - A vector.
///
/// * `inc_y` - Increment between elements of `y`.
///
/// # Returns
///
/// The dot product of `x` and `y`.
pub fn ak_sdot(n: usize, x: &[SFLOAT], inc_x: usize, y: &[SFLOAT], inc_y: usize) -> SFLOAT {
    let mut result: SFLOAT = 0.0;
    let c: usize;

    if n < 1 {
        return result;
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        if is_x86_feature_detected!("avx") {
            result += unsafe { sdot::mk_sdot_s8el_avx(c, x, y) };
        } else {
            result += templates::mk_dot_s8el(c, x, y);
        }
    } else {
        c = n & !1;
        result += templates::mk_dot_ns2el(c, x, inc_x, y, inc_y);
    }

    for i in c..n {
        result += x[i * inc_x] * y[i * inc_y];
    }

    result
}
