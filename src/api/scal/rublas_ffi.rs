use std::ffi::{c_double, c_float, c_size_t};

use crate::utils::types::{cf32, cf64};

use super::scal;

#[unsafe(no_mangle)]
pub extern "C" fn scal_f32(n: c_size_t, alpha: c_float, x: *mut c_float, inc_x: c_size_t) -> () {
    let n: usize = n as usize;
    let alpha: f32 = alpha as f32;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(x, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[unsafe(no_mangle)]
pub extern "C" fn scal_f64(n: c_size_t, alpha: c_double, x: *mut c_double, inc_x: c_size_t) -> () {
    let n: usize = n as usize;
    let alpha: f64 = alpha as f64;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(x, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[unsafe(no_mangle)]
pub extern "C" fn scal_cf32(
    n: c_size_t,
    alpha: *const c_float,
    x: *mut c_float,
    inc_x: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: cf32 = cf32::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(x as *mut cf32, n * inc_x) };

    scal(n, alpha, x, inc_x);
}

#[unsafe(no_mangle)]
pub extern "C" fn scal_cf64(
    n: c_size_t,
    alpha: *const c_double,
    x: *mut c_double,
    inc_x: c_size_t,
) -> () {
    let n: usize = n as usize;
    let alpha: cf64 = cf64::from(alpha);
    let inc_x: usize = inc_x as usize;
    let x: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(x as *mut cf64, n * inc_x) };

    scal(n, alpha, x, inc_x);
}
