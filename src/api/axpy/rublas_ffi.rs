use std::ffi::{c_double, c_float, c_size_t};

use crate::utils::types::{cf32, cf64};

use super::axpy;

#[unsafe(no_mangle)]
pub extern "C" fn axpy_f32(
    n: c_size_t,
    alpha: c_float,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: f32 = alpha as f32;
    let inc_x: usize = inc_x as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_f64(
    n: c_size_t,
    alpha: c_double,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: f64 = alpha as f64;
    let inc_x: usize = inc_x as usize;
    let x: &[f64] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_cf32(
    n: c_size_t,
    alpha: *const c_float,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: cf32 = cf32::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(x as *const cf32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf32, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_cf64(
    n: c_size_t,
    alpha: *const c_double,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: cf64 = cf64::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(x as *const cf64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf64, n * inc_y) };

    axpy(n, alpha, x, inc_x, y, inc_y);
}
