use super::swap;
use crate::utils::types::{cf32, cf64};
use std::ffi::{c_double, c_float, c_int, c_void};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_sswap(
    N: c_int,
    X: *mut c_float,
    incX: c_int,
    Y: *mut c_float,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(Y, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_dswap(
    N: c_int,
    X: *mut c_double,
    incX: c_int,
    Y: *mut c_double,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(X, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(Y, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_cswap(
    N: c_int,
    X: *mut c_void,
    incX: c_int,
    Y: *mut c_void,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(X as *mut cf32, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(Y as *mut cf32, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_zswap(
    N: c_int,
    X: *mut c_void,
    incX: c_int,
    Y: *mut c_void,
    incY: c_int,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(X as *mut cf64, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(Y as *mut cf64, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}
