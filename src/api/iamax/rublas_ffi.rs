use super::iamax;
use crate::utils::types::{cf32, cf64};
use std::ffi::{c_double, c_float, c_size_t};

#[unsafe(no_mangle)]
pub extern "C" fn iamax_f32(n: c_size_t, x: *const c_float, inc_x: c_size_t) -> c_size_t {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn iamax_f64(n: c_size_t, x: *const c_double, inc_x: c_size_t) -> c_size_t {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[f64] = unsafe { std::slice::from_raw_parts(x, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn iamax_cf32(n: c_size_t, x: *const c_float, inc_x: c_size_t) -> c_size_t {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(x as *const cf32, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn iamax_cf64(n: c_size_t, x: *const c_double, inc_x: c_size_t) -> c_size_t {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(x as *const cf64, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}
