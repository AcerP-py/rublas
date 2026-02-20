use std::ops::{AddAssign, Mul};

use crate::kernel::k_axpy;

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

pub fn axpy<T>(n: usize, alpha: T, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: AddAssign + Copy + Mul<Output = T>,
{
    if n <= 0 {
        return ();
    }

    k_axpy(n, alpha, x, inc_x, y, inc_y);
}
