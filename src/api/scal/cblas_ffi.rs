use super::scal;
use crate::utils::types::{cf32, cf64};
use std::ffi::{c_double, c_float, c_int, c_void};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_sscal(N: c_int, alpha: c_float, X: *mut c_float, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: f32 = alpha as f32;
    let inc_x: usize = incX as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(X, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_dscal(N: c_int, alpha: c_double, X: *mut c_double, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: f64 = alpha as f64;
    let inc_x: usize = incX as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(X, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_cscal(N: c_int, alpha: *const c_void, X: *const c_void, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: cf32 = cf32::from(alpha as *const c_float);
    let inc_x: usize = incX as usize;
    let x: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(X as *mut cf32, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_zscal(N: c_int, alpha: *const c_void, X: *const c_void, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: cf64 = cf64::from(alpha as *const c_double);
    let inc_x: usize = incX as usize;
    let x: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(X as *mut cf64, n * inc_x) };

    scal(n, alpha, x, inc_x);
}
