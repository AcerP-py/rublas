use super::rotm;
use crate::utils::enums::RotmFlag;
use std::ffi::{c_double, c_float, c_int, c_size_t};

#[unsafe(no_mangle)]
pub extern "C" fn rotm_f32(
    n: c_size_t,
    x: *mut c_float,
    inc_x: c_size_t,
    y: *mut c_float,
    inc_y: c_size_t,
    flag: c_int,
    params: *const c_float,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(x as *mut f32, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(y as *mut f32, n * inc_y) };
    let flag: RotmFlag = match flag as i32 {
        0 => RotmFlag::General,
        1 => RotmFlag::Diagonal,
        2 => RotmFlag::Shear,
        3 => RotmFlag::Identity,
        _ => panic!("Invalid flag value in P array"),
    };
    let params: &[f32] = unsafe { std::slice::from_raw_parts(params, 5) };

    rotm(n, x, inc_x, y, inc_y, flag, params);
}

#[unsafe(no_mangle)]
pub extern "C" fn rotm_f64(
    n: c_size_t,
    x: *mut c_double,
    inc_x: c_size_t,
    y: *mut c_double,
    inc_y: c_size_t,
    flag: c_int,
    params: *const c_double,
) -> () {
    let n: usize = n as usize;
    let inc_x: usize = inc_x as usize;
    let x: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(x as *mut f64, n * inc_x) };
    let inc_y: usize = inc_y as usize;
    let y: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(y as *mut f64, n * inc_y) };
    let flag: RotmFlag = match flag as i32 {
        0 => RotmFlag::General,
        1 => RotmFlag::Diagonal,
        2 => RotmFlag::Shear,
        3 => RotmFlag::Identity,
        _ => panic!("Invalid flag value in P array"),
    };
    let params: &[f64] = unsafe { std::slice::from_raw_parts(params, 5) };

    rotm(n, x, inc_x, y, inc_y, flag, params);
}
