mod kernel;

/*
 * BLAS implementation in Rust. It generally folows https://www.netlib.org/blas/cblas.h
 * but does differ in a few places where it makes sense.
 */

/*
 * ===========================================================================
 * Level 1 BLAS
 * ===========================================================================
 */


// axpy
#[allow(unused_variables, dead_code)]
pub fn saxpy(n: usize, alpha: f32, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn daxpy(n: usize, alpha: f64, x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn caxpy(n: usize, alpha: &[f32], x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn zaxpy(n: usize, alpha: &[f64], x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> () { }


// copy
#[allow(unused_variables, dead_code)]
pub fn scopy(n: usize, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn dcopy(n: usize, x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn ccopy(n: usize, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn zcopy(n: usize, x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> () { }


// dot
#[allow(unused_variables, dead_code)]
pub fn sdsdot(n: usize, alpha: f32, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 { 0.0 }

#[allow(unused_variables, dead_code)]
pub fn dsdot(n: usize, x: &[f64], inc_x: usize, y: &[f64], inc_y: usize) -> f64 { 0.0 }

pub fn sdot(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 { kernel::k_sdot(n, x, inc_x, y, inc_y) }

#[allow(unused_variables, dead_code)]
pub fn ddot(n: usize, x: &[f64], inc_x: usize, y: &[f64], inc_y: usize) -> f64 { 0.0 }

#[allow(unused_variables, dead_code)]
pub fn cdotu(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> Vec<f32> { vec![0.0, 0.0] }

#[allow(unused_variables, dead_code)]
pub fn cdotc(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> Vec<f32> { vec![0.0, 0.0] }

#[allow(unused_variables, dead_code)]
pub fn zdotu(n: usize, x: &[f64], inc_x: usize, y: &[f64], inc_y: usize) -> Vec<f64> { vec![0.0, 0.0] }

#[allow(unused_variables, dead_code)]
pub fn zdotc(n: usize, x: &[f64], inc_x: usize, y: &[f64], inc_y: usize) -> Vec<f64> { vec![0.0, 0.0] }

// nrm2
#[allow(unused_variables, dead_code)]
pub fn snrm2(n: usize, x: &[f32], inc_x: usize) -> f32 { 0.0 }

#[allow(unused_variables, dead_code)]
pub fn dnrm2(n: usize, x: &[f64], inc_x: usize) -> f64 { 0.0 }

#[allow(unused_variables, dead_code)]
pub fn scnrm2(n: usize, x: &[f32], inc_x: usize) -> f32 { 0.0 }

#[allow(unused_variables, dead_code)]
pub fn dznrm2(n: usize, x: &[f64], inc_x: usize) -> f64 { 0.0 }


// scal
#[allow(unused_variables, dead_code)]
pub fn sscal(n: usize, alpha: f32, x: &mut [f32], inc_x: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn dscal(n: usize, alpha: f64, x: &mut [f64], inc_x: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn cscal(n: usize, alpha: &[f32], x: &mut [f32], inc_x: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn zscal(n: usize, alpha: &[f64], x: &mut [f64], inc_x: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn csscal(n: usize, alpha: f32, x: &mut [f32], inc_x: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn zdcal(n: usize, alpha: f64, x: &mut [f64], inc_x: usize) -> () { }


// swap
#[allow(unused_variables, dead_code)]
pub fn sswap(n: usize, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn dswap(n: usize, x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn cswap(n: usize, x: &[f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> () { }

#[allow(unused_variables, dead_code)]
pub fn zswap(n: usize, x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> () { }

#[cfg(test)]
mod tests {
    use super::*;

    const X_S: [f32; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    const Y_S: [f32; 10] = [21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];

    #[test]
    fn sdot_basic() {
        let mut result: f32;

        result = sdot(10, &X_S, 1, &Y_S, 1);
        assert_eq!(result, 1485.0);

        result = sdot(4, &X_S, 1, &Y_S, 1);
        assert_eq!(result, 230.0);

        result = sdot(0, &X_S, 1, &Y_S, 1);
        assert_eq!(result, 0.0);

        result = sdot(5, &X_S, 2, &Y_S, 2);
        assert_eq!(result, 665.0);
    }
    
}
