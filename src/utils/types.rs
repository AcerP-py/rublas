use std::ops::{Add, Div, Mul, Sub};

use super::traits::{BLASAbsoluteValue, Conjigatable, Zeroable};

use super::complex::Complex;

// ===========================================================================
// usize
// ===========================================================================
impl BLASAbsoluteValue for usize {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for usize {
    fn conj(self) -> Self { self }
}

impl Zeroable for usize {
    fn zero() -> Self { 0usize }
}

// ===========================================================================
// u8
// ===========================================================================
impl BLASAbsoluteValue for u8 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for u8 {
    fn conj(self) -> Self { self }
}

impl Zeroable for u8 {
    fn zero() -> Self { 0u8 }
}

// ===========================================================================
// u16
// ===========================================================================
impl BLASAbsoluteValue for u16 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for u16 {
    fn conj(self) -> Self { self }
}

impl Zeroable for u16 {
    fn zero() -> Self { 0u16 }
}

// ===========================================================================
// u32
// ===========================================================================
impl BLASAbsoluteValue for u32 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for u32 {
    fn conj(self) -> Self { self }
}

impl Zeroable for u32 {
    fn zero() -> Self { 0u32 }
}

// ===========================================================================
// u64
// ===========================================================================
impl BLASAbsoluteValue for u64 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for u64 {
    fn conj(self) -> Self { self }
}

impl Zeroable for u64 {
    fn zero() -> Self { 0u64 }
}

// ===========================================================================
// u128
// ===========================================================================
impl BLASAbsoluteValue for u128 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for u128 {
    fn conj(self) -> Self { self }
}

impl Zeroable for u128 {
    fn zero() -> Self { 0u128 }
}

// ===========================================================================
// isize
// ===========================================================================
impl BLASAbsoluteValue for isize {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for isize {
    fn conj(self) -> Self { self }
}

impl Zeroable for isize {
    fn zero() -> Self { 0isize }
}

// ===========================================================================
// i8
// ===========================================================================
impl BLASAbsoluteValue for i8 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for i8 {
    fn conj(self) -> Self { self }
}

impl Zeroable for i8 {
    fn zero() -> Self { 0i8 }
}

// ===========================================================================
// i16
// ===========================================================================
impl BLASAbsoluteValue for i16 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for i16 {
    fn conj(self) -> Self { self }
}

impl Zeroable for i16 {
    fn zero() -> Self { 0i16 }
}

// ===========================================================================
// i32
// ===========================================================================
impl BLASAbsoluteValue for i32 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for i32 {
    fn conj(self) -> Self { self }
}

impl Zeroable for i32 {
    fn zero() -> Self { 0i32 }
}

// ===========================================================================
// i64
// ===========================================================================
impl BLASAbsoluteValue for i64 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for i64 {
    fn conj(self) -> Self { self }
}

impl Zeroable for i64 {
    fn zero() -> Self { 0i64 }
}

// ===========================================================================
// i128
// ===========================================================================
impl BLASAbsoluteValue for i128 {
    type Output = Self;
    fn blas_abs(self) -> Self { self }
}

impl Conjigatable for i128 {
    fn conj(self) -> Self { self }
}

impl Zeroable for i128 {
    fn zero() -> Self { 0i128 }
}

// ===========================================================================
// f32
// ===========================================================================
impl BLASAbsoluteValue for f32 {
    type Output = f32;
    fn blas_abs(self) -> Self::Output { self.abs() }
}

impl Conjigatable for f32 {
    fn conj(self) -> Self { self }
}

impl Zeroable for f32 {
    fn zero() -> Self { 0.0f32 }
}

// ===========================================================================
// f64
// ===========================================================================
impl BLASAbsoluteValue for f64 {
    type Output = f64;
    fn blas_abs(self) -> Self::Output { self.abs() }
}

impl Conjigatable for f64 {
    fn conj(self) -> Self { self }
}

impl Zeroable for f64 {
    fn zero() -> Self { 0.0f64 }
}

// ===========================================================================
// cf32
// ===========================================================================
#[allow(non_camel_case_types)]
pub type cf32 = Complex<f32>;

impl BLASAbsoluteValue for cf32 {
    type Output = f32;
    fn blas_abs(self) -> Self::Output { self.real().abs() + self.imag().abs() }
}

impl Add<cf32> for f32 {
    type Output = cf32;

    fn add(self, rhs: cf32) -> Self::Output { cf32::new(self + rhs.real(), rhs.imag()) }
}

impl Div<cf32> for f32 {
    type Output = cf32;

    fn div(self, rhs: cf32) -> Self::Output {
        let denom: f32 = rhs.real() * rhs.real() + rhs.imag() * rhs.imag();
        cf32::new((self * rhs.real()) / denom, (-self * rhs.imag()) / denom)
    }
}

impl Mul<cf32> for f32 {
    type Output = cf32;

    fn mul(self, rhs: cf32) -> Self::Output { cf32::new(self * rhs.real(), self * rhs.imag()) }
}

impl Sub<cf32> for f32 {
    type Output = cf32;

    fn sub(self, rhs: cf32) -> Self::Output { cf32::new(self - rhs.real(), -rhs.imag()) }
}

pub static I32: cf32 = cf32::new(0.0f32, 1.0f32);

// ===========================================================================
// c_float_complex
// ===========================================================================
#[allow(non_camel_case_types)]
pub type c_float_complex = cf32;

// ===========================================================================
// cf64
// ===========================================================================
#[allow(non_camel_case_types)]
pub type cf64 = Complex<f64>;

impl From<cf32> for cf64 {
    fn from(value: cf32) -> Self { Self::new(value.real() as f64, value.imag() as f64) }
}

impl BLASAbsoluteValue for cf64 {
    type Output = f64;
    fn blas_abs(self) -> Self::Output { self.real().abs() + self.imag().abs() }
}

impl Add<cf64> for f64 {
    type Output = cf64;

    fn add(self, rhs: cf64) -> Self::Output { cf64::new(self + rhs.real(), rhs.imag()) }
}

impl Div<cf64> for f64 {
    type Output = cf64;

    fn div(self, rhs: cf64) -> Self::Output {
        let denom: f64 = rhs.real() * rhs.real() + rhs.imag() * rhs.imag();
        cf64::new((self * rhs.real()) / denom, (-self * rhs.imag()) / denom)
    }
}

impl Mul<cf64> for f64 {
    type Output = cf64;

    fn mul(self, rhs: cf64) -> Self::Output { cf64::new(self * rhs.real(), self * rhs.imag()) }
}

impl Sub<cf64> for f64 {
    type Output = cf64;

    fn sub(self, rhs: cf64) -> Self::Output { cf64::new(self - rhs.real(), -rhs.imag()) }
}

pub static I64: cf64 = cf64::new(0.0f64, 1.0f64);

// ===========================================================================
// c_double_complex
// ===========================================================================
#[allow(non_camel_case_types)]
pub type c_double_complex = cf64;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cf32_basic() {
        assert_eq!(I32.real(), 0.0f32);
        assert_eq!(I32.imag(), 1.0f32);

        let a: cf32 = cf32::new(2.0f32, 3.0f32);
        let mut r: cf32;

        // math for f32 * Complex<f32>
        // Add
        r = 2.0f32 + a;
        assert_eq!(r.real(), 4.0f32);
        assert_eq!(r.imag(), 3.0f32);

        // Div
        r = 26.0f32 / a;
        assert_eq!(r.real(), 4.0f32);
        assert_eq!(r.imag(), -6.0f32);

        // Mul
        r = 2.0f32 * a;
        assert_eq!(r.real(), 4.0f32);
        assert_eq!(r.imag(), 6.0f32);

        // Sub
        r = 2.0f32 - a;
        assert_eq!(r.real(), 0.0f32);
        assert_eq!(r.imag(), -3.0f32);
    }

    #[test]
    fn cf64_basic() {
        assert_eq!(I64.real(), 0.0f64);
        assert_eq!(I64.imag(), 1.0f64);

        let a: cf64 = cf64::new(2.0f64, 3.0f64);
        let mut r: cf64;

        // math for f32 * Complex<f32>
        // Add
        r = 2.0f64 + a;
        assert_eq!(r.real(), 4.0f64);
        assert_eq!(r.imag(), 3.0f64);

        // Div
        r = 26.0f64 / a;
        assert_eq!(r.real(), 4.0f64);
        assert_eq!(r.imag(), -6.0f64);

        // Mul
        r = 2.0f64 * a;
        assert_eq!(r.real(), 4.0f64);
        assert_eq!(r.imag(), 6.0f64);

        // Sub
        r = 2.0f64 - a;
        assert_eq!(r.real(), 0.0f64);
        assert_eq!(r.imag(), -3.0f64);
    }
}
