use std::ffi::{c_double, c_float, c_int, c_void};

use crate::utils::types::{CFloat32, CFloat64, Float32, Float64};

use super::axpy;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_saxpy(
    N: c_int,
    alpha: c_float,
    X: *const c_float,
    incX: c_int,
    Y: *mut c_float,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let alpha: Float32 = alpha as Float32;
    let inc_x: usize = incX as usize;
    let x: &[Float32] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [Float32] = unsafe { std::slice::from_raw_parts_mut(Y, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_daxpy(
    N: c_int,
    alpha: c_double,
    X: *const c_double,
    incX: c_int,
    Y: *mut c_double,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let alpha: Float64 = alpha as Float64;
    let inc_x: usize = incX as usize;
    let x: &[Float64] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [Float64] = unsafe { std::slice::from_raw_parts_mut(Y, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_caxpy(
    N: c_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: c_int,
    Y: *mut c_void,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let alpha: CFloat32 = CFloat32::from(alpha as *const c_float);
    let inc_x: usize = incX as usize;
    let x: &[CFloat32] = unsafe { std::slice::from_raw_parts(X as *const CFloat32, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [CFloat32] =
        unsafe { std::slice::from_raw_parts_mut(Y as *mut CFloat32, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_zaxpy(
    N: c_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: c_int,
    Y: *mut c_void,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let alpha: CFloat64 = CFloat64::from(alpha as *const c_double);
    let inc_x: usize = incX as usize;
    let x: &[CFloat64] = unsafe { std::slice::from_raw_parts(X as *const CFloat64, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [CFloat64] =
        unsafe { std::slice::from_raw_parts_mut(Y as *mut CFloat64, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}
