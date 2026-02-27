use crate::{
    kernel::k_iamax,
    utils::types::{BLASAbsoluteValue, Zeroable},
};

#[cfg(feature = "cblas")]
mod cblas_ffi;
#[cfg(feature = "c_cpp")]
mod rublas_ffi;

pub fn iamax<T>(n: usize, x: &[T], inc_x: usize) -> usize
where
    T: BLASAbsoluteValue + Copy,
{
    if n <= 0 {
        return usize::zero();
    }

    k_iamax(n, x, inc_x)
}
