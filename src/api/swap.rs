use crate::{kernel::k_swap, utils::traits::Zeroable};

//#[cfg(feature = "cblas")]
mod cblas_ffi;
//#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// swap (x <-> y)
///
/// # Arguments
/// * `n` - length of vectors `x` and `y`
/// * `x` - a vector
/// * `inc_x` - increment between elements of `x`
/// * `y` - a vector
/// * `inc_y` - increment between elements of `y`
pub fn swap<T>(n: usize, x: &mut [T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: Copy + Zeroable,
{
    if n == 0 {
        return ();
    }

    k_swap(n, x, inc_x, y, inc_y);
}
