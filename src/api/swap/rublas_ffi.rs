use super::swap;
use crate::utils::types::{cf32, cf64};
use std::ffi::{c_double, c_float, c_size_t};

#[unsafe(no_mangle)]
pub extern "C" fn swap_f32(
    n: c_size_t,
    x: *mut c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn swap_f64(
    n: c_size_t,
    x: *mut c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(y, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn swap_cf32(
    n: c_size_t,
    x: *mut c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(x as *mut cf32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf32, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn swap_cf64(
    n: c_size_t,
    x: *mut c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(x as *mut cf64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf64, n * inc_y) };

    swap(n, x, inc_x, y, inc_y);
}
