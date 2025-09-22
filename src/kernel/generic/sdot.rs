use crate::utils::types::SFLOAT;

/// mk (micro-kernel) _
/// sdot (SFLOAT dot product) _
/// ns2el (non-sequential 2 element)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 2 and within bounds).
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
pub fn mk_sdot_ns2el(n: usize, x: &[SFLOAT], inc_x: usize, y: &[SFLOAT], inc_y: usize) -> SFLOAT {
    let mut result: SFLOAT = 0.0;

    let mut x1: usize = 0;
    let mut y1: usize = 0;
    let mut x2: usize = n / 2 * inc_x;
    let mut y2: usize = n / 2 * inc_y;

    for _ in 0..(n / 2) {
        result += x[x1] * y[y1] + x[x2] * y[y2];
        x1 += inc_x;
        x2 += inc_x;
        y1 += inc_y;
        y2 += inc_y;
    }

    result
}

/// mk (micro-kernel) _
/// sdot (SFLOAT dot product) _
/// s8el (sequential 8 element)
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
pub fn mk_sdot_s8el(n: usize, x: &[SFLOAT], y: &[SFLOAT]) -> SFLOAT {
    let mut result: SFLOAT = 0.0;

    for i in (0..n).step_by(8) {
        result += x[i] * y[i]
            + x[i + 1] * y[i + 1]
            + x[i + 2] * y[i + 2]
            + x[i + 3] * y[i + 3]
            + x[i + 4] * y[i + 4]
            + x[i + 5] * y[i + 5]
            + x[i + 6] * y[i + 6]
            + x[i + 7] * y[i + 7];
    }

    result
}
