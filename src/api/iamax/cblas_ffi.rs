use super::iamax;
use crate::utils::types::{cf32, cf64};
use std::ffi::{c_double, c_float, c_int, c_size_t, c_void};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_isamax(N: c_int, X: *const c_float, incX: c_int) -> c_size_t {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[f32] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_idamax(N: c_int, X: *const c_double, incX: c_int) -> c_size_t {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[f64] = unsafe { std::slice::from_raw_parts(X, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_icamax(N: c_int, X: *const c_void, incX: c_int) -> c_size_t {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[cf32] = unsafe { std::slice::from_raw_parts(X as *const cf32, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_izamax(N: c_int, X: *const c_void, incX: c_int) -> c_size_t {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &[cf64] = unsafe { std::slice::from_raw_parts(X as *const cf64, n * inc_x) };

    iamax(n, x, inc_x) as c_size_t
}
