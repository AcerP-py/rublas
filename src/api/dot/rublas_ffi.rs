use super::dot;
use crate::utils::types::{c_double_complex, c_float_complex, cf32, cf64};
use std::ffi::{c_double, c_float, c_size_t};

#[unsafe(no_mangle)]
pub extern "C" fn dot_f32(
    n: c_size_t,
    x: *const c_float,
    inc_x: c_size_t,
    y: *const c_float,
    inc_y: c_size_t,
) -> c_float {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &[f32] = unsafe { std::slice::from_raw_parts(y, n * inc_y) };

    let result: f32 = dot(n, x, inc_x, y, inc_y, false);

    result as c_float
}

#[unsafe(no_mangle)]
pub extern "C" fn dot_f32_as_f64(
    n: c_size_t,
    x: *const c_float,
    inc_x: c_size_t,
    y: *const c_float,
    inc_y: c_size_t,
) -> c_double {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &[f32] = unsafe { std::slice::from_raw_parts(y, n * inc_y) };

    let result: f64 = dot(n, x, inc_x, y, inc_y, false);

    result as c_double
}

#[unsafe(no_mangle)]
pub extern "C" fn dot_f64(
    n: c_size_t,
    x: *const c_double,
    inc_x: c_size_t,
    y: *const c_double,
    inc_y: c_size_t,
) -> c_double {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[f64] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &[f64] = unsafe { std::slice::from_raw_parts(y, n * inc_y) };

    let result: f64 = dot(n, x, inc_x, y, inc_y, false);

    result as c_double
}

#[unsafe(no_mangle)]
pub extern "C" fn dot_cf32(
    n: c_size_t,
    x: *const c_float,
    inc_x: c_size_t,
    y: *const c_float,
    inc_y: c_size_t,
) -> c_float_complex {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(x as *const cf32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &[cf32] = unsafe { std::slice::from_raw_parts(y as *const cf32, n * inc_y) };

    let result: cf32 = dot(n, x, inc_x, y, inc_y, false);

    c_float_complex::from(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn dot_cf32_conj(
    n: c_size_t,
    x: *const c_float,
    inc_x: c_size_t,
    y: *const c_float,
    inc_y: c_size_t,
) -> c_float_complex {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(x as *const cf32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &[cf32] = unsafe { std::slice::from_raw_parts(y as *const cf32, n * inc_y) };

    let result: cf32 = dot(n, x, inc_x, y, inc_y, true);

    c_float_complex::from(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn dot_cf64(
    n: c_size_t,
    x: *const c_double,
    inc_x: c_size_t,
    y: *const c_double,
    inc_y: c_size_t,
) -> c_double_complex {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(x as *const cf64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &[cf64] = unsafe { std::slice::from_raw_parts(y as *const cf64, n * inc_y) };

    let result: cf64 = dot(n, x, inc_x, y, inc_y, false);

    c_double_complex::from(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn dot_cf64_conj(
    n: c_size_t,
    x: *const c_double,
    inc_x: c_size_t,
    y: *const c_double,
    inc_y: c_size_t,
) -> c_double_complex {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(x as *const cf64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &[cf64] = unsafe { std::slice::from_raw_parts(y as *const cf64, n * inc_y) };

    let result: cf64 = dot(n, x, inc_x, y, inc_y, true);

    c_double_complex::from(result)
}
