pub fn mk_sdot_seq_8el(n: usize, x: &[f32], y: &[f32]) -> f32 {
    let mut result: f32 = 0.0;

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

pub fn mk_sdot_2el(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 {
    let mut result: f32 = 0.0;
    
    let mut x1: usize = 0;
    let mut y1: usize = 0;
    let mut x2: usize = n / 2 * inc_x;
    let mut y2: usize = n / 2 * inc_y;

    for _ in 0..(n / 2) {
        result += x[x1] * y[y1]
            + x[x2] * y[y2];
        x1 += inc_x;
        x2 += inc_x;
        y1 += inc_y;
        y2 += inc_y;
    }

    result
}
