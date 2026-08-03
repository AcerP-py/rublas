use super::rotm;
use crate::utils::enums::RotmFlag;
use std::ffi::{c_double, c_float, c_int};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_srotm(
    N: c_int,
    X: *mut c_float,
    incX: c_int,
    Y: *mut c_float,
    incY: c_int,
    P: *const c_float,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(X as *mut f32, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(Y as *mut f32, n * inc_y) };
    let params_array: &[f32] = unsafe { std::slice::from_raw_parts(P as *const f32, 5) };
    let flag: RotmFlag = match params_array[0] {
        -1.0 => RotmFlag::General,
        0.0 => RotmFlag::Diagonal,
        1.0 => RotmFlag::Shear,
        -2.0 => RotmFlag::Identity,
        _ => panic!("Invalid flag value in P array"),
    };

    rotm(n, x, inc_x, y, inc_y, flag, &params_array[1..5]);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn cblas_drotm(
    N: c_int,
    X: *mut c_double,
    incX: c_int,
    Y: *mut c_double,
    incY: c_int,
    P: *const c_double,
) -> () {
    let n: usize = N as usize;
    let inc_x: usize = incX as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(X as *mut f64, n * inc_x) };
    let inc_y: usize = incY as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(Y as *mut f64, n * inc_y) };
    let params_array: &[f64] = unsafe { std::slice::from_raw_parts(P as *const f64, 5) };
    let flag = match params_array[0] {
        -1.0 => RotmFlag::General,
        0.0 => RotmFlag::Diagonal,
        1.0 => RotmFlag::Shear,
        -2.0 => RotmFlag::Identity,
        _ => panic!("Invalid flag value in P array"),
    };

    rotm(n, x, inc_x, y, inc_y, flag, &params_array[1..5]);
}
