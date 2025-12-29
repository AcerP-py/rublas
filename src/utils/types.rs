use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

// ===========================================================================
// useful traits
// ===========================================================================
pub trait Zeroable {
    fn zero() -> Self;
}

// ===========================================================================
// usize
// ===========================================================================

impl Zeroable for usize {
    fn zero() -> Self { 0 }
}

// ===========================================================================
// Float32
// ===========================================================================

pub type Float32 = f32;

impl Zeroable for Float32 {
    fn zero() -> Self { 0.0 }
}

// ===========================================================================
// Float64
// ===========================================================================

pub type Float64 = f64;

impl Zeroable for Float64 {
    fn zero() -> Self { 0.0 }
}

// ===========================================================================
// CFLOAT
// ===========================================================================
#[derive(Clone, Copy, Debug)]
pub struct CFloat32 {
    real: Float32,
    imag: Float32,
}

impl CFloat32 {
    pub fn new(real: Float32, imag: Float32) -> Self { Self { real, imag } }

    pub fn real(&self) -> Float32 { self.real }

    pub fn imag(&self) -> Float32 { self.imag }
}

impl Display for CFloat32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.imag < 0.0 {
            write!(f, "{} - {}i", self.real, self.imag.abs())
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

impl From<*const Float32> for CFloat32 {
    fn from(value: *const Float32) -> Self {
        assert!(!value.is_null(), "Null pointer passed to CFloat32::from");

        unsafe {
            Self {
                real: *value,
                imag: *value.add(1),
            }
        }
    }
}

impl Add<Self> for CFloat32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl AddAssign<Self> for CFloat32 {
    fn add_assign(&mut self, rhs: Self) {
        self.real = self.real + rhs.real;
        self.imag = self.imag + rhs.imag;
    }
}

impl Sub<Self> for CFloat32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul<Self> for CFloat32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Div<Self> for CFloat32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom: Float32 = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / denom,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / denom,
        }
    }
}

impl Zeroable for CFloat32 {
    fn zero() -> Self {
        Self {
            real: 0.0,
            imag: 0.0,
        }
    }
}

#[macro_export]
macro_rules! cfloat32 {
    // real + imag * I
    ($real:literal + $imag:literal * I) => {
        CFloat32::new($real, $imag)
    };

    // real - imag * I
    ($real:literal - $imag:literal * I) => {
        CFloat32::new($real, -$imag)
    };

    // real
    ($real:literal) => {
        CFloat32::new($real, 0.0)
    };

    // imag * I
    ($imag:literal * I) => {
        CFloat32::new(0.0, $imag)
    };
}

#[macro_export]
macro_rules! cfloat32_vec {
    // empty vector
    [] => {
        Vec::<CFloat32>::new()
    };

    // explicit vector
    [ $( $real:literal $sign:tt $imag:literal * I ),* $(,)? ] => {
        vec![
            $(
                CFloat32::new($real, 0.0 $sign $imag)
            ),*
        ]
    };

    // complex; number of repeats
    [ $real:literal $sign:tt $imag:literal * I; $n_repeats:expr ] => {
        vec![CFLOAT::new($real, 0.0 $sign $imag); $n_repeats]
    };
}

// ===========================================================================
// CFloat64
// ===========================================================================
#[derive(Clone, Copy, Debug)]
pub struct CFloat64 {
    real: Float64,
    imag: Float64,
}

impl CFloat64 {
    pub fn new(real: Float64, imag: Float64) -> Self { Self { real, imag } }

    pub fn real(&self) -> Float64 { self.real }

    pub fn imag(&self) -> Float64 { self.imag }
}

impl Display for CFloat64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.imag < 0.0 {
            write!(f, "{} - {}i", self.real, self.imag.abs())
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

impl From<*const Float64> for CFloat64 {
    fn from(value: *const Float64) -> Self {
        assert!(!value.is_null(), "Null pointer passed to CFloat64::from");

        unsafe {
            Self {
                real: *value,
                imag: *value.add(1),
            }
        }
    }
}

impl Add<Self> for CFloat64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl AddAssign<Self> for CFloat64 {
    fn add_assign(&mut self, rhs: Self) {
        self.real = self.real + rhs.real;
        self.imag = self.imag + rhs.imag;
    }
}

impl Sub<Self> for CFloat64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul<Self> for CFloat64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Div<Self> for CFloat64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom: Float64 = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / denom,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / denom,
        }
    }
}

impl Zeroable for CFloat64 {
    fn zero() -> Self {
        Self {
            real: 0.0,
            imag: 0.0,
        }
    }
}

#[macro_export]
macro_rules! cfloat64 {
    // real + imag * I
    ($real:literal + $imag:literal * I) => {
        CFloat64::new($real, $imag)
    };

    // real - imag * I
    ($real:literal - $imag:literal * I) => {
        CFloat64::new($real, -$imag)
    };

    // real
    ($real:literal) => {
        CFloat64::new($real, 0.0)
    };

    // imag * I
    ($imag:literal * I) => {
        CFloat64::new(0.0, $imag)
    };
}

#[macro_export]
macro_rules! cfloat64_vec {
    // empty vector
    [] => {
        Vec::<CFloat64>::new()
    };

    // explicit vector
    [ $( $real:literal $sign:tt $imag:literal * I ),* $(,)? ] => {
        vec![
            $(
                CFloat64::new($real, 0.0 $sign $imag)
            ),*
        ]
    };

    // complex; number of repeats
    [ $real:literal $sign:tt $imag:literal * I; $n_repeats:expr ] => {
        vec![CFloat64::new($real, 0.0 $sign $imag); $n_repeats]
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn float32_basic() {
        let x: Float32 = 3.45;
        assert_eq!(x, 3.45);
    }

    #[test]
    fn float64_basic() {
        let x: Float64 = 3.45;
        assert_eq!(x, 3.45);
    }

    #[test]
    fn cfloat32_basic() {
        let mut x: CFloat32 = cfloat32!(2.3 + 8.2 * I);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 8.2);

        x = cfloat32!(2.3 - 8.2 * I);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), -8.2);

        x = cfloat32!(2.3);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 0.0);

        x = cfloat32!(8.2 * I);
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), 8.2);

        x = cfloat32!(-8.2 * I);
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), -8.2);
    }

    #[test]
    fn cfloat32_vector() {
        let x: Vec<CFloat32> = cfloat32_vec![
            1.0 + 2.0 * I,
            3.0 - 4.0 * I,
            5.0 + 0.0 * I, // we must put these numbers in full form until
            0.0 + 6.0 * I, // we can create a better macro
            0.0 - 7.0 * I, // to handle different type inputs in a vector
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
    fn cfloat32_math() {
        let x: CFloat32 = cfloat32!(346.0 - 78.0 * I);
        let y: CFloat32 = cfloat32!(1.0 - 43.0 * I);

        // Add
        let mut z: CFloat32 = x + y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // AddAssign
        z = cfloat32!(346.0 - 78.0 * I);
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
    fn cfloat64_basic() {
        let mut x: CFloat64 = cfloat64!(2.3 + 8.2 * I);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 8.2);

        x = cfloat64!(2.3 - 8.2 * I);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), -8.2);

        x = cfloat64!(2.3);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 0.0);

        x = cfloat64!(8.2 * I);
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), 8.2);

        x = cfloat64!(-8.2 * I);
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), -8.2);
    }

    #[test]
    fn cfloat64_vector() {
        let x: Vec<CFloat64> = cfloat64_vec![
            1.0 + 2.0 * I,
            3.0 - 4.0 * I,
            5.0 + 0.0 * I, // we must put these numbers in full form until
            0.0 + 6.0 * I, // we can create a better macro
            0.0 - 7.0 * I, // to handle different type inputs in a vector
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
    fn cfloat64_math() {
        let x: CFloat64 = cfloat64!(346.0 - 78.0 * I);
        let y: CFloat64 = cfloat64!(1.0 - 43.0 * I);

        // Add
        let mut z: CFloat64 = x + y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // AddAssign
        z = cfloat64!(346.0 - 78.0 * I);
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
