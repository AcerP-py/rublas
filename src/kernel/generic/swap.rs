use crate::utils::traits::Zeroable;

// ***************************************************************************
// ***************************************************************************
// COPY trait and generic implementations
// ***************************************************************************
// ***************************************************************************

pub trait SWAP
where
    Self: Sized,
{
    /// mk (micro-kernel) _
    /// swap (x <-> y) _
    /// 2nsel (consumes elements by 2 non-sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 2)
    /// * `x` - a vector
    /// * `inc_x` - increment between elements of `x`
    /// * `y` - a vector
    /// * `inc_y` - increment between elements of `y`
    fn mk_swap_2nsel(n: usize, x: &mut [Self], inc_x: usize, y: &mut [Self], inc_y: usize) -> ();

    /// mk (micro-kernel) _
    /// swap (x <-> y) _
    /// 8sel (consumes elements by 8 sequential elements)
    ///
    /// # Arguments
    /// * `n` - length of vectors `x` and `y` (must be divisible by 8)
    /// * `x` - a vector
    /// * `y` - a vector
    fn mk_swap_8sel(n: usize, x: &mut [Self], y: &mut [Self]) -> ();
}

impl<T> SWAP for T
where
    T: Copy + Zeroable,
{
    default fn mk_swap_2nsel(
        n: usize,
        x: &mut [Self],
        inc_x: usize,
        y: &mut [Self],
        inc_y: usize,
    ) -> () {
        let mut x1: usize = 0;
        let mut y1: usize = 0;
        let mut x2: usize = n / 2 * inc_x;
        let mut y2: usize = n / 2 * inc_y;

        let mut shelf1: T;
        let mut shelf2: T;

        for _ in 0..(n / 2) {
            shelf1 = y[y1];
            shelf2 = y[y2];

            y[y1] = x[x1];
            y[y2] = x[x2];

            x[x1] = shelf1;
            x[x2] = shelf2;

            x1 += inc_x;
            x2 += inc_x;
            y1 += inc_y;
            y2 += inc_y;
        }
    }

    default fn mk_swap_8sel(n: usize, x: &mut [Self], y: &mut [Self]) -> () {
        let mut shelf: [T; 8] = [T::zero(); 8];

        for i in (0..n).step_by(8) {
            shelf[i % 8] = y[i];
            shelf[i % 8 + 1] = y[i + 1];
            shelf[i % 8 + 2] = y[i + 2];
            shelf[i % 8 + 3] = y[i + 3];
            shelf[i % 8 + 4] = y[i + 4];
            shelf[i % 8 + 5] = y[i + 5];
            shelf[i % 8 + 6] = y[i + 6];
            shelf[i % 8 + 7] = y[i + 7];

            y[i] = x[i];
            y[i + 1] = x[i + 1];
            y[i + 2] = x[i + 2];
            y[i + 3] = x[i + 3];
            y[i + 4] = x[i + 4];
            y[i + 5] = x[i + 5];
            y[i + 6] = x[i + 6];
            y[i + 7] = x[i + 7];

            x[i] = shelf[i % 8];
            x[i + 1] = shelf[i % 8 + 1];
            x[i + 2] = shelf[i % 8 + 2];
            x[i + 3] = shelf[i % 8 + 3];
            x[i + 4] = shelf[i % 8 + 4];
            x[i + 5] = shelf[i % 8 + 5];
            x[i + 6] = shelf[i % 8 + 6];
            x[i + 7] = shelf[i % 8 + 7];
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
    fn mk_swap_2nsel_basic() {
        let mut x: Vec<i32> = vec![1; 8];
        let mut y: Vec<i32> = vec![2; 8];
        let mut inc: usize;

        inc = 1;
        i32::mk_swap_2nsel(4, &mut x, inc, &mut y, inc);
        assert_eq!(x, vec![2, 2, 2, 2, 1, 1, 1, 1]);
        assert_eq!(y, vec![1, 1, 1, 1, 2, 2, 2, 2]);

        inc = 2;
        i32::mk_swap_2nsel(4, &mut x, inc, &mut y, inc);
        assert_eq!(x, vec![1, 2, 1, 2, 2, 1, 2, 1]);
        assert_eq!(y, vec![2, 1, 2, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn mk_copy_8sel_basic() {
        let mut x: Vec<i32> = vec![1; 16];
        let mut y: Vec<i32> = vec![2; 16];

        i32::mk_swap_8sel(16, &mut x, &mut y);
        assert_eq!(x, vec![2; 16]);
        assert_eq!(y, vec![1; 16]);
    }
}
