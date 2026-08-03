// ***************************************************************************
// ***************************************************************************
// COPY trait and generic implementation
// ***************************************************************************
// ***************************************************************************

pub trait COPY
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// copy (y = x) _
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 2)
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    /// * `y` - a vector
    /// * `inc_y` - increment between elements of `y`
    fn mk_copy_2nsel(n: usize, x: &[Self], inc_x: usize, y: &mut [Self], inc_y: usize) -> ();

    /// mk (micro-kernel) _
    /// copy (y = x) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    fn mk_copy_8sel(n: usize, x: &[Self], y: &mut [Self]) -> ();
}

impl<T> COPY for T
where
    T: Copy,
{
    default fn mk_copy_2nsel(
        n: usize,
        x: &[Self],
        inc_x: usize,
        y: &mut [Self],
        inc_y: usize,
    ) -> () {
        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;
        let mut y2: usize = n / 2 * inc_y;

        for _ in 0..(n / 2) {
            y[y1] = x[x1];
            y[y2] = x[x2];
            x1 += inc_x;
            x2 += inc_x;
            y1 += inc_y;
            y2 += inc_y;
        }
    }

    default fn mk_copy_8sel(n: usize, x: &[Self], y: &mut [Self]) -> () {
        for i in (0..n).step_by(8) {
            y[i] = x[i];
            y[i + 1] = x[i + 1];
            y[i + 2] = x[i + 2];
            y[i + 3] = x[i + 3];
            y[i + 4] = x[i + 4];
            y[i + 5] = x[i + 5];
            y[i + 6] = x[i + 6];
            y[i + 7] = x[i + 7];
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
    fn mk_copy_2nsel_basic() {
        let x: Vec<i32> = vec![1; 8];
        let mut y: Vec<i32>;
        let mut inc: usize;

        y = vec![2; 4];
        inc = 1;
        i32::mk_copy_2nsel(4, &x, inc, &mut y, inc);
        assert_eq!(y, vec![1; 4]);

        y = vec![2; 8];
        inc = 2;
        i32::mk_copy_2nsel(4, &x, inc, &mut y, inc);
        assert_eq!(y, vec![1, 2, 1, 2, 1, 2, 1, 2]);
    }

    #[test]
    fn mk_copy_8sel_basic() {
        let x: Vec<i32> = vec![1; 16];
        let mut y: Vec<i32> = vec![2; 16];

        i32::mk_copy_8sel(16, &x, &mut y);
        assert_eq!(y, x);
    }
}
