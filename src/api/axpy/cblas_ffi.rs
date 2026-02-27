use std::ffi::{c_double, c_float, c_int, c_void};

use crate::utils::types::{cf32, cf64};

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
    let alpha: f32 = alpha as f32;
    let inc_x: usize = incX as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(Y, n * inc_y) };

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
    let alpha: f64 = alpha as f64;
    let inc_x: usize = incX as usize;
    let x: &[f64] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(Y, n * inc_y) };

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
    let alpha: cf32 = cf32::from(alpha as *const c_float);
    let inc_x: usize = incX as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(X as *const cf32, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(Y as *mut cf32, n * inc_y) };

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
    let alpha: cf64 = cf64::from(alpha as *const c_double);
    let inc_x: usize = incX as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(X as *const cf64, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(Y as *mut cf64, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}
