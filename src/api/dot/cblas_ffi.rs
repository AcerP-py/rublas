use super::dot;
use crate::utils::types::{cf32, cf64};
use std::ffi::{c_double, c_float, c_int, c_void};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_sdot(
    N: c_int,
    X: *const c_float,
    incX: c_int,
    Y: *const c_float,
    incY: c_int,
) -> c_float {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[f32] = unsafe { std::slice::from_raw_parts(Y, n * inc_y) };

    let result: f32 = dot(n, x, inc_x, y, inc_y, false);

    result as c_float
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_sdsdot(
    N: c_int,
    alpha: c_float,
    X: *const c_float,
    incX: c_int,
    Y: *const c_float,
    incY: c_int,
) -> c_float {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[f32] = unsafe { std::slice::from_raw_parts(Y, n * inc_y) };

    let mut result: f64 = dot(n, x, inc_x, y, inc_y, false);
    result += alpha as f64;

    result as f32 as c_float
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_dsdot(
    N: c_int,
    X: *const c_float,
    incX: c_int,
    Y: *const c_float,
    incY: c_int,
) -> c_double {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[f32] = unsafe { std::slice::from_raw_parts(Y, n * inc_y) };

    let result: f64 = dot(n, x, inc_x, y, inc_y, false);

    result as c_double
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_ddot(
    N: c_int,
    X: *const c_double,
    incX: c_int,
    Y: *const c_double,
    incY: c_int,
) -> c_double {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[f64] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[f64] = unsafe { std::slice::from_raw_parts(Y, n * inc_y) };

    let result: f64 = dot(n, x, inc_x, y, inc_y, false);

    result as c_double
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_cdotu_sub(
    N: c_int,
    X: *const c_void,
    incX: c_int,
    Y: *const c_void,
    incY: c_int,
    dotu: *mut c_void,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(X as *const cf32, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[cf32] = unsafe { std::slice::from_raw_parts(Y as *const cf32, n * inc_y) };
    let dot_u_ptr: *mut cf32 = dotu as *mut cf32;

    let result: cf32 = dot(n, x, inc_x, y, inc_y, false);

    unsafe {
        std::ptr::write(dot_u_ptr, result);
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_cdotc_sub(
    N: c_int,
    X: *const c_void,
    incX: c_int,
    Y: *const c_void,
    incY: c_int,
    dotc: *mut c_void,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(X as *const cf32, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[cf32] = unsafe { std::slice::from_raw_parts(Y as *const cf32, n * inc_y) };
    let dot_c_ptr: *mut cf32 = dotc as *mut cf32;

    let result: cf32 = dot(n, x, inc_x, y, inc_y, true);

    unsafe {
        std::ptr::write(dot_c_ptr, result);
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_zdotu_sub(
    N: c_int,
    X: *const c_void,
    incX: c_int,
    Y: *const c_void,
    incY: c_int,
    dotu: *mut c_void,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(X as *const cf64, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[cf64] = unsafe { std::slice::from_raw_parts(Y as *const cf64, n * inc_y) };
    let dot_u_ptr: *mut cf64 = dotu as *mut cf64;

    let result: cf64 = dot(n, x, inc_x, y, inc_y, false);

    unsafe {
        std::ptr::write(dot_u_ptr, result);
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_zdotc_sub(
    N: c_int,
    X: *const c_void,
    incX: c_int,
    Y: *const c_void,
    incY: c_int,
    dotc: *mut c_void,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(X as *const cf64, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &[cf64] = unsafe { std::slice::from_raw_parts(Y as *const cf64, n * inc_y) };
    let dot_c_ptr: *mut cf64 = dotc as *mut cf64;

    let result: cf64 = dot(n, x, inc_x, y, inc_y, true);

    unsafe {
        std::ptr::write(dot_c_ptr, result);
    }
}
