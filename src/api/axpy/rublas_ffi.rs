use std::ffi::{c_double, c_float, c_size_t};

use crate::utils::types::{CFloat32, CFloat64, Float32, Float64};

use super::axpy;

#[unsafe(no_mangle)]
pub extern "C" fn axpy_float32(
    n: c_size_t,
    alpha: c_float,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: Float32 = alpha as Float32;
    let inc_x: usize = inc_x as usize;
    let x: &[Float32] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [Float32] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_float64(
    n: c_size_t,
    alpha: c_double,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: Float64 = alpha as Float64;
    let inc_x: usize = inc_x as usize;
    let x: &[Float64] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [Float64] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_cfloat32(
    n: c_size_t,
    alpha: *const c_float,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: CFloat32 = CFloat32::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &[CFloat32] = unsafe { std::slice::from_raw_parts(x as *const CFloat32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [CFloat32] =
        unsafe { std::slice::from_raw_parts_mut(y as *mut CFloat32, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_cfloat64(
    n: c_size_t,
    alpha: *const c_double,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: CFloat64 = CFloat64::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &[CFloat64] = unsafe { std::slice::from_raw_parts(x as *const CFloat64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [CFloat64] =
        unsafe { std::slice::from_raw_parts_mut(y as *mut CFloat64, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}
