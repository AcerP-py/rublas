use crate::kernel::k_copy;

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

/// copy (y = x)
///
/// # Arguments
///
/// * `n` - length of vectors `x` and `y`
///
/// * `x` - a vector
///
/// * `inc_x` - increment between elements of `x`
///
/// * `y` - a vector
///
/// * `inc_y` - increment between elements of `y`
pub fn copy<T>(n: usize, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: Copy,
{
    if n <= 0 {
        return ();
    }

    k_copy(n, x, inc_x, y, inc_y);
}
