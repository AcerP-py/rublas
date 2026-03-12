use std::ffi::{c_double, c_float, c_size_t};

use crate::utils::types::{cf32, cf64};

use super::copy;

#[unsafe(no_mangle)]
pub extern "C" fn copy_f32(
    n: c_size_t,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    copy(n, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn copy_f64(
    n: c_size_t,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[f64] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    copy(n, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn copy_cf32(
    n: c_size_t,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(x as *const cf32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf32, n * inc_y) };

    copy(n, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn copy_cf64(
    n: c_size_t,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(x as *const cf64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf64, n * inc_y) };

    copy(n, x, inc_x, y, inc_y);
}
