use super::rot;
use crate::utils::types::{c_double_complex, c_float_complex, cf32, cf64};
use std::ffi::{c_double, c_float, c_int, c_void};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_srot(
    N: c_int,
    X: *mut c_float,
    incX: c_int,
    Y: *mut c_float,
    incY: c_int,
    c: c_float,
    s: c_float,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(X as *mut f32, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(Y as *mut f32, n * inc_y) };
    let c: f32 = c as f32;
    let s: f32 = s as f32;

    rot(n, x, inc_x, y, inc_y, c, s);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_drot(
    N: c_int,
    X: *mut c_double,
    incX: c_int,
    Y: *mut c_double,
    incY: c_int,
    c: c_double,
    s: c_double,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(X as *mut f64, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(Y as *mut f64, n * inc_y) };
    let c: f64 = c as f64;
    let s: f64 = s as f64;

    rot(n, x, inc_x, y, inc_y, c, s);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_csrot(
    N: c_int,
    X: *mut c_void,
    incX: c_int,
    Y: *mut c_void,
    incY: c_int,
    c: c_float,
    s: c_float,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [cf32] = unsafe {
        std::slice::from_raw_parts_mut(X as *mut c_float_complex as *mut cf32, n * inc_x)
    };
    let inc_y: usize = incY as usize;
    let y: &mut [cf32] = unsafe {
        std::slice::from_raw_parts_mut(Y as *mut c_float_complex as *mut cf32, n * inc_y)
    };
    let c: f32 = c as f32;
    let s: f32 = s as f32;

    rot(n, x, inc_x, y, inc_y, c, s);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_zdrot(
    N: c_int,
    X: *mut c_void,
    incX: c_int,
    Y: *mut c_void,
    incY: c_int,
    c: c_double,
    s: c_double,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [cf64] = unsafe {
        std::slice::from_raw_parts_mut(X as *mut c_double_complex as *mut cf64, n * inc_x)
    };
    let inc_y: usize = incY as usize;
    let y: &mut [cf64] = unsafe {
        std::slice::from_raw_parts_mut(Y as *mut c_double_complex as *mut cf64, n * inc_y)
    };
    let c: f64 = c as f64;
    let s: f64 = s as f64;

    rot(n, x, inc_x, y, inc_y, c, s);
}
