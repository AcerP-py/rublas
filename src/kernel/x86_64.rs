/*
 * declare mods for each operation
 */

mod sdot;

/*
 * architecture kernels for each operation
 */

pub fn ak_sdot(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 {
    let mut result: f32 = 0.0;

    let c: usize;

    if n < 1 {
        return result;
    }

    if inc_x == 1 && inc_y == 1 {
        c = n & !7;
        if is_x86_feature_detected!("avx2") {
            result += unsafe { sdot::mk_sdot_seq_8el_avx2(c, x, y) }
        } else {
            result += sdot::mk_sdot_seq_8el(c, x, y)
        }
    } else {
        c = n & !1;
        result += sdot::mk_sdot_2el(c, x, inc_x, y, inc_y)
    }

    for i in c..n {
        result += x[i * inc_x] * y[i * inc_y];
    }

    result
}
