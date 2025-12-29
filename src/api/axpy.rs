use std::{
    ffi::{c_double, c_float, c_size_t},
    ops::{AddAssign, Mul},
};

use crate::{
    kernel::k_axpy,
    utils::types::{CFloat32, CFloat64, Float32, Float64},
};

pub fn axpy<T>(n: usize, alpha: T, x: &[T], inc_x: usize, y: &mut [T], inc_y: usize) -> ()
where
    T: AddAssign + Copy + Mul<Output = T>,
{
    if n <= 0 {
        return ();
    }

    k_axpy(n, alpha, x, inc_x, y, inc_y);
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_float32(
    n: c_size_t,
    alpha: c_float,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    unsafe {
        let x_slice: &[Float32] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [Float32] = std::slice::from_raw_parts_mut(y, n * inc_y);
        axpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_float64(
    n: c_size_t,
    alpha: c_double,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    unsafe {
        let x_slice: &[Float64] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [Float64] = std::slice::from_raw_parts_mut(y, n * inc_y);
        axpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_cfloat32(
    n: c_size_t,
    alpha: *const c_float,
    x: *const c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
) -> () {
    let alpha: CFloat32 = CFloat32::from(alpha);
    let x: *const CFloat32 = x as *const CFloat32;
    let y: *mut CFloat32 = y as *mut CFloat32;

    unsafe {
        let x_slice: &[CFloat32] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [CFloat32] = std::slice::from_raw_parts_mut(y, n * inc_y);
        axpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn axpy_cfloat64(
    n: c_size_t,
    alpha: *const c_double,
    x: *const c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
) -> () {
    let alpha: CFloat64 = CFloat64::from(alpha);
    let x: *const CFloat64 = x as *const CFloat64;
    let y: *mut CFloat64 = y as *mut CFloat64;

    unsafe {
        let x_slice: &[CFloat64] = std::slice::from_raw_parts(x, n * inc_x);
        let y_slice: &mut [CFloat64] = std::slice::from_raw_parts_mut(y, n * inc_y);
        axpy(n, alpha, x_slice, inc_x, y_slice, inc_y);
    }
}
