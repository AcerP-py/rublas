use std::{
    any::type_name,
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

// ===========================================================================
// useful traits
// ===========================================================================
pub trait Zeroable {
    fn zero() -> Self;
}

// ===========================================================================
// Complex
// ===========================================================================
#[derive(Clone, Copy, Debug)]
pub struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> Complex<T>
where
    T: Copy,
{
    pub fn new(real: T, imag: T) -> Self { Self { real, imag } }

    pub fn real(&self) -> T { self.real }

    pub fn imag(&self) -> T { self.imag }
}

// misc traits
impl<T> Display for Complex<T>
where
    T: Copy + Display + Neg<Output = T> + PartialOrd + Zeroable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.imag < T::zero() {
            write!(f, "{} - {}i", self.real, -self.imag)
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

impl<T> From<*const T> for Complex<T>
where
    T: Copy,
{
    fn from(value: *const T) -> Self {
        assert!(
            !value.is_null(),
            "Null pointer passed to Complex<{}>::from",
            type_name::<T>()
        );

        unsafe {
            Self {
                real: *value,
                imag: *value.add(1),
            }
        }
    }
}

impl<T> Zeroable for Complex<T>
where
    T: Zeroable,
{
    fn zero() -> Self {
        Self {
            real: T::zero(),
            imag: T::zero(),
        }
    }
}

// math functions for self
impl<T> Add<Self> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<T> AddAssign<Self> for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.real = self.real + rhs.real;
        self.imag = self.imag + rhs.imag;
    }
}

impl<T> Div<Self> for Complex<T>
where
    T: Add<Output = T> + Copy + Div<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom: T = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / denom,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / denom,
        }
    }
}

impl<T> Mul<Self> for Complex<T>
where
    T: Add<Output = T> + Copy + Mul<Output = T> + Sub<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl<T> Sub<Self> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

// math functions for other types
impl<T> Add<T> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    fn add(self, rhs: T) -> Self::Output {
        Complex::<T> {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}

impl<T> Mul<T> for Complex<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Complex<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Complex::<T> {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}

impl<T> Sub<T> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Complex<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Complex::<T> {
            real: self.real - rhs,
            imag: self.imag,
        }
    }
}

// ===========================================================================
// usize
// ===========================================================================

impl Zeroable for usize {
    fn zero() -> Self { 0 }
}

// ===========================================================================
// f32
// ===========================================================================

impl Zeroable for f32 {
    fn zero() -> Self { 0.0 }
}

// ===========================================================================
// f64
// ===========================================================================

impl Zeroable for f64 {
    fn zero() -> Self { 0.0 }
}

// ===========================================================================
// cf32
// ===========================================================================
#[allow(non_camel_case_types)]
pub type cf32 = Complex<f32>;

impl Add<cf32> for f32 {
    type Output = cf32;

    fn add(self, rhs: cf32) -> Self::Output {
        cf32 {
            real: self + rhs.real,
            imag: rhs.imag,
        }
    }
}

impl Mul<cf32> for f32 {
    type Output = cf32;

    fn mul(self, rhs: cf32) -> Self::Output {
        cf32 {
            real: self * rhs.real,
            imag: self * rhs.imag,
        }
    }
}

impl Sub<cf32> for f32 {
    type Output = cf32;

    fn sub(self, rhs: cf32) -> Self::Output {
        cf32 {
            real: self - rhs.real,
            imag: -rhs.imag,
        }
    }
}

pub static I32: cf32 = cf32 {
    real: 0.0,
    imag: 1.0,
};

// ===========================================================================
// cf64
// ===========================================================================
#[allow(non_camel_case_types)]
pub type cf64 = Complex<f64>;

impl Add<cf64> for f64 {
    type Output = cf64;

    fn add(self, rhs: cf64) -> Self::Output {
        cf64 {
            real: self + rhs.real,
            imag: rhs.imag,
        }
    }
}

impl Mul<cf64> for f64 {
    type Output = cf64;

    fn mul(self, rhs: cf64) -> Self::Output {
        cf64 {
            real: self * rhs.real,
            imag: self * rhs.imag,
        }
    }
}

impl Sub<cf64> for f64 {
    type Output = cf64;

    fn sub(self, rhs: cf64) -> Self::Output {
        cf64 {
            real: self - rhs.real,
            imag: -rhs.imag,
        }
    }
}

pub static I64: cf64 = cf64 {
    real: 0.0,
    imag: 1.0,
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f32_basic() {
        let x: f32 = 3.45;
        assert_eq!(x, 3.45);
    }

    #[test]
    fn f64_basic() {
        let x: f64 = 3.45;
        assert_eq!(x, 3.45);
    }

    #[test]
    fn cf32_basic() {
        let mut x: cf32 = 2.3 + 8.2 * I32;
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 8.2);

        x = 2.3 - 8.2 * I32;
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), -8.2);

        x = 2.3 + 0.0 * I32;
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 0.0);

        x = 8.2 * I32;
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), 8.2);

        x = -8.2 * I32;
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), -8.2);
    }

    #[test]
    fn cf32_vector() {
        let x: Vec<cf32> = vec![
            1.0 + 2.0 * I32,
            3.0 - 4.0 * I32,
            5.0 + 0.0 * I32, // we have to add the '+ 0.0 * I' for now
            6.0 * I32,
            -7.0 * I32,
        ];

        assert_eq!(x[0].real(), 1.0);
        assert_eq!(x[0].imag(), 2.0);

        assert_eq!(x[1].real(), 3.0);
        assert_eq!(x[1].imag(), -4.0);

        assert_eq!(x[2].real(), 5.0);
        assert_eq!(x[2].imag(), 0.0);

        assert_eq!(x[3].real(), 0.0);
        assert_eq!(x[3].imag(), 6.0);

        assert_eq!(x[4].real(), 0.0);
        assert_eq!(x[4].imag(), -7.0);
    }

    #[test]
    fn cf32_math() {
        let x: cf32 = 346.0 - 78.0 * I32;
        let y: cf32 = 1.0 - 43.0 * I32;

        // Add
        let mut z: cf32 = x + y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // AddAssign
        z = 346.0 - 78.0 * I32;
        z += y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // Sub
        z = x - y;
        assert_eq!(z.real(), 345.0);
        assert_eq!(z.imag(), -35.0);

        // Mul
        z = x * y;
        assert_eq!(z.real(), -3008.0);
        assert_eq!(z.imag(), -14956.0);

        // Div
        z = x / y;
        assert_eq!(z.real(), 2.0);
        assert_eq!(z.imag(), 8.0);
    }

    #[test]
    fn cf64_basic() {
        let mut x: cf64 = 2.3 + 8.2 * I64;
        //x = 8.2 * I;
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 8.2);

        x = 2.3 - 8.2 * I64;
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), -8.2);

        x = 2.3 + 0.0 * I64;
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 0.0);

        x = 8.2 * I64;
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), 8.2);

        x = -8.2 * I64;
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), -8.2);
    }

    #[test]
    fn cf64_vector() {
        let x: Vec<cf64> = vec![
            1.0 + 2.0 * I64,
            3.0 - 4.0 * I64,
            5.0 + 0.0 * I64, // we have to add the '+ 0.0 * I' for now
            6.0 * I64,
            -7.0 * I64,
        ];

        assert_eq!(x[0].real(), 1.0);
        assert_eq!(x[0].imag(), 2.0);

        assert_eq!(x[1].real(), 3.0);
        assert_eq!(x[1].imag(), -4.0);

        assert_eq!(x[2].real(), 5.0);
        assert_eq!(x[2].imag(), 0.0);

        assert_eq!(x[3].real(), 0.0);
        assert_eq!(x[3].imag(), 6.0);

        assert_eq!(x[4].real(), 0.0);
        assert_eq!(x[4].imag(), -7.0);
    }

    #[test]
    fn cf64_math() {
        let x: cf64 = 346.0 - 78.0 * I64;
        let y: cf64 = 1.0 - 43.0 * I64;

        // Add
        let mut z: cf64 = x + y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // AddAssign
        z = 346.0 - 78.0 * I64;
        z += y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // Sub
        z = x - y;
        assert_eq!(z.real(), 345.0);
        assert_eq!(z.imag(), -35.0);

        // Mul
        z = x * y;
        assert_eq!(z.real(), -3008.0);
        assert_eq!(z.imag(), -14956.0);

        // Div
        z = x / y;
        assert_eq!(z.real(), 2.0);
        assert_eq!(z.imag(), 8.0);
    }
}
