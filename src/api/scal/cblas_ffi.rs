use std::ffi::{c_double, c_float, c_int, c_void};

use crate::utils::types::{CFloat32, CFloat64, Float32, Float64};

use super::scal;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_sscal(N: c_int, alpha: c_float, X: *mut c_float, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: Float32 = alpha as Float32;
    let inc_x: usize = incX as usize;
    let x: &mut [Float32] = unsafe { std::slice::from_raw_parts_mut(X, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_dscal(N: c_int, alpha: c_double, X: *mut c_double, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: Float64 = alpha as Float64;
    let inc_x: usize = incX as usize;
    let x: &mut [Float64] = unsafe { std::slice::from_raw_parts_mut(X, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_cscal(N: c_int, alpha: *const c_void, X: *const c_void, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: CFloat32 = CFloat32::from(alpha as *const c_float);
    let inc_x: usize = incX as usize;
    let x: &mut [CFloat32] =
        unsafe { std::slice::from_raw_parts_mut(X as *mut CFloat32, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_zscal(N: c_int, alpha: *const c_void, X: *const c_void, incX: c_int) -> () {
    let n: usize = N as usize;
    let alpha: CFloat64 = CFloat64::from(alpha as *const c_double);
    let inc_x: usize = incX as usize;
    let x: &mut [CFloat64] =
        unsafe { std::slice::from_raw_parts_mut(X as *mut CFloat64, n * inc_x) };

    scal(n, alpha, x, inc_x);
}
