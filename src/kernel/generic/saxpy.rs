/// mk (micro-kernel) _
/// saxpy (f32 y = y + ax) _
/// ns2el (non-sequential 2 element)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 2 and within bounds).
///
/// * `alpha` - scaling factor for `x`.
///
/// * `x` - A vector.
///
/// * `inc_x` - Increment between elements of `x`.
///
/// * `y` - A vector.
///
/// * `inc_y` - Increment between elements of `y`.
pub fn mk_saxpy_ns2el(
    n: usize,
    alpha: f32,
    x: &[f32],
    inc_x: usize,
    y: &mut [f32],
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

/// mk (micro-kernel) _
/// saxpy (f32 y = y + ax) _
/// s8el (sequential 8 element)
///
/// # Arguments
///
/// * `n` - Length of vectors `x` and `y`
/// (must be divisible by 8 and within bounds).
///
/// * `alpha` - scaling factor for `x`.
///
/// * `x` - A vector.
///
/// * `y` - A vector.
pub fn mk_saxpy_s8el(n: usize, alpha: f32, x: &[f32], y: &mut [f32]) -> () {
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
