use std::ffi::{c_double, c_float, c_size_t};

use crate::utils::types::{CFloat32, CFloat64, Float32, Float64};

use super::scal;

#[unsafe(no_mangle)]
pub extern "C" fn scal_float32(
    n: c_size_t,
    alpha: c_float,
    x: *mut c_float,
    inc_x: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: Float32 = alpha as Float32;
    let inc_x: usize = inc_x as usize;
    let x: &mut [Float32] = unsafe { std::slice::from_raw_parts_mut(x, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[unsafe(no_mangle)]
pub extern "C" fn scal_float64(
    n: c_size_t,
    alpha: c_double,
    x: *mut c_double,
    inc_x: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: Float64 = alpha as Float64;
    let inc_x: usize = inc_x as usize;
    let x: &mut [Float64] = unsafe { std::slice::from_raw_parts_mut(x, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[unsafe(no_mangle)]
pub extern "C" fn scal_cfloat32(
    n: c_size_t,
    alpha: *const c_float,
    x: *mut c_float,
    inc_x: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: CFloat32 = CFloat32::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &mut [CFloat32] =
        unsafe { std::slice::from_raw_parts_mut(x as *mut CFloat32, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[unsafe(no_mangle)]
pub extern "C" fn scal_cfloat64(
    n: c_size_t,
    alpha: *const c_double,
    x: *mut c_double,
    inc_x: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: CFloat64 = CFloat64::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &mut [CFloat64] =
        unsafe { std::slice::from_raw_parts_mut(x as *mut CFloat64, n * inc_x) };

    scal(n, alpha, x, inc_x);
}
