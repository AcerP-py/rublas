use super::traits::{Conjugatable, Zeroable};
use std::{
    any::type_name,
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

// ===========================================================================
// Complex
// ===========================================================================
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> Complex<T>
where
    T: Copy,
{
    pub const fn new(real: T, imag: T) -> Self { Self { real, imag } }

    pub const fn real(&self) -> T { self.real }

    pub const fn imag(&self) -> T { self.imag }
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

impl<T> From<*const Complex<T>> for Complex<T>
where
    T: Copy,
{
    fn from(value: *const Complex<T>) -> Self {
        assert!(
            !value.is_null(),
            "Null pointer passed to Complex<{}>::from",
            type_name::<T>()
        );

        unsafe { *value }
    }
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool { self.real == other.real && self.imag == other.imag }
}

impl<T> Conjugatable for Complex<T>
where
    T: Neg<Output = T>,
{
    fn conj(self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
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

// math functions for Complex<T>
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

// math functions for T
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

impl<T> Div<T> for Complex<T>
where
    T: Add<Output = T> + Copy + Div<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    type Output = Complex<T>;

    fn div(self, rhs: T) -> Self::Output {
        let denom: T = rhs * rhs;
        Self {
            real: (self.real * rhs) / denom,
            imag: (self.imag * rhs) / denom,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn complex_basic() {
        let mut r: Complex<i32>;

        // create Complex<T>
        let a: Complex<i32> = Complex::<i32>::new(-14, 2);
        assert_eq!(a.real(), -14);
        assert_eq!(a.imag(), 2);

        let b: Complex<i32> = Complex::<i32>::new(4, 3);
        assert_eq!(b.real(), 4);
        assert_eq!(b.imag(), 3);

        // Display
        // Not tested here.

        // From *const Complex<T>
        let a_ptr: *const Complex<i32> = &a;
        r = Complex::<i32>::from(a_ptr);
        assert_eq!(r, a);

        // PartialEq
        assert_eq!(a, a);
        assert_ne!(a, b);

        // Conjugatable
        r = a.conj();
        assert_eq!(r.real(), -14);
        assert_eq!(r.imag(), -2);

        // Zeroable
        // Not tested here.

        // math with Complex<T>
        // Add
        r = a + b;
        assert_eq!(r.real(), -10);
        assert_eq!(r.imag(), 5);

        // AddAssign
        r += a;
        assert_eq!(r.real(), -24);
        assert_eq!(r.imag(), 7);

        // Div
        r = a / b;
        assert_eq!(r.real(), -2);
        assert_eq!(r.imag(), 2);

        // Mul
        r = a * b;
        assert_eq!(r.real(), -62);
        assert_eq!(r.imag(), -34);

        // Sub
        r = a - b;
        assert_eq!(r.real(), -18);
        assert_eq!(r.imag(), -1);

        // math with T
        // Add
        r = a + 2;
        assert_eq!(r.real(), -12);
        assert_eq!(r.imag(), 2);

        // Div
        r = a / 2;
        assert_eq!(r.real(), -7);
        assert_eq!(r.imag(), 1);

        // Mul
        r = a * 2;
        assert_eq!(r.real(), -28);
        assert_eq!(r.imag(), 4);

        // Sub
        r = a - 2;
        assert_eq!(r.real(), -16);
        assert_eq!(r.imag(), 2);
    }
}
