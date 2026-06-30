use super::rot;
use crate::utils::types::{c_double_complex, c_float_complex, cf32, cf64};
use std::ffi::{c_double, c_float, c_size_t};

#[unsafe(no_mangle)]
pub extern "C" fn rot_f32(
    n: c_size_t,
    x: *mut c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
    c: c_float,
    s: c_float,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(x as *mut f32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(y as *mut f32, n * inc_y) };
    let c: f32 = c as f32;
    let s: f32 = s as f32;

    rot(n, x, inc_x, y, inc_y, c, s);
}

#[unsafe(no_mangle)]
pub extern "C" fn rot_f64(
    n: c_size_t,
    x: *mut c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
    c: c_double,
    s: c_double,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(x as *mut f64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(y as *mut f64, n * inc_y) };
    let c: f64 = c as f64;
    let s: f64 = s as f64;

    rot(n, x, inc_x, y, inc_y, c, s);
}

#[unsafe(no_mangle)]
pub extern "C" fn rot_cf32(
    n: c_size_t,
    x: *mut c_float_complex,
    inc_x: c_size_t,
    y: *mut c_float_complex,
    inc_y: c_size_t,
    c: c_float,
    s: c_float,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(x as *mut cf32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf32] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf32, n * inc_y) };
    let c: f32 = c as f32;
    let s: f32 = s as f32;

    rot(n, x, inc_x, y, inc_y, c, s);
}

#[unsafe(no_mangle)]
pub extern "C" fn rot_cf64(
    n: c_size_t,
    x: *mut c_double_complex,
    inc_x: c_size_t,
    y: *mut c_double_complex,
    inc_y: c_size_t,
    c: c_double,
    s: c_double,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(x as *mut cf64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [cf64] = unsafe { std::slice::from_raw_parts_mut(y as *mut cf64, n * inc_y) };
    let c: f64 = c as f64;
    let s: f64 = s as f64;

    rot(n, x, inc_x, y, inc_y, c, s);
}
