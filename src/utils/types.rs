use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

pub trait Zeroable {
    fn zero() -> Self;
}

// ===========================================================================
// SFLOAT
// ===========================================================================
pub type SFLOAT = f32;

impl Zeroable for SFLOAT {
    fn zero() -> Self { 0.0 }
}

// ===========================================================================
// DFLOAT
// ===========================================================================
pub type DFLOAT = f64;

impl Zeroable for DFLOAT {
    fn zero() -> Self { 0.0 }
}

// ===========================================================================
// CFLOAT
// ===========================================================================
#[derive(Clone, Copy)]
pub struct CFLOAT {
    real: SFLOAT,
    imag: SFLOAT,
}

impl CFLOAT {
    pub fn new(real: SFLOAT, imag: SFLOAT) -> Self { Self { real, imag } }

    pub fn real(&self) -> SFLOAT { self.real }

    pub fn imag(&self) -> SFLOAT { self.imag }
}

impl Display for CFLOAT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Add<Self> for CFLOAT {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl AddAssign<Self> for CFLOAT {
    fn add_assign(&mut self, rhs: Self) {
        self.real = self.real + rhs.real;
        self.imag = self.imag + rhs.imag;
    }
}

impl Sub<Self> for CFLOAT {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul<Self> for CFLOAT {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Div<Self> for CFLOAT {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom: SFLOAT = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / denom,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / denom,
        }
    }
}

impl Zeroable for CFLOAT {
    fn zero() -> Self {
        CFLOAT {
            real: 0.0,
            imag: 0.0,
        }
    }
}

#[macro_export]
macro_rules! cfloat {
    // real + imag * I
    ($real:literal + $imag:literal * I) => {
        CFLOAT::new($real, $imag)
    };

    // real - imag * I
    ($real:literal - $imag:literal * I) => {
        CFLOAT::new($real, -$imag)
    };

    // real
    ($real:literal) => {
        CFLOAT::new($real, 0.0)
    };

    // imag * I
    ($imag:literal * I) => {
        CFLOAT::new(0.0, $imag)
    };
}

#[macro_export]
macro_rules! cfloat_vec {
    // empty vector
    [] => {
        Vec::<CFLOAT>::new()
    };

    // explicit vector
    [ $( $expr:expr ),* $(,)? ] => {
        (::defile::defile! {
            vec![
                $(
                    cfloat!(@$expr)
                ),*
            ]
        })
    };

    // complex; repeat
    [ $expr:expr; $num_repeats:literal ] => {
        (::defile::defile! {
            vec![cfloat!(@$expr); $num_repeats]
        })
    };
    [ $expr:expr; $num_repeats:ident ] => {
        (::defile::defile! {
            vec![cfloat!(@$expr); $num_repeats]
        })
    };
    [ $expr:expr; $num_repeats:expr ] => {
        (::defile::defile! {
            vec![cfloat!(@$expr); $num_repeats]
        })
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sfloat_basic() {
        let x: SFLOAT = 3.45;
        assert_eq!(x, 3.45);
    }

    #[test]
    fn dfloat_basic() {
        let x: DFLOAT = 3.45;
        assert_eq!(x, 3.45);
    }

    #[test]
    fn cfloat_basic() {
        let mut x: CFLOAT = cfloat!(2.3 + 8.2 * I);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 8.2);

        x = cfloat!(2.3 - 8.2 * I);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), -8.2);

        x = cfloat!(2.3);
        assert_eq!(x.real(), 2.3);
        assert_eq!(x.imag(), 0.0);

        x = cfloat!(8.2 * I);
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), 8.2);

        x = cfloat!(-8.2 * I);
        assert_eq!(x.real(), 0.0);
        assert_eq!(x.imag(), -8.2);
    }

    #[test]
    fn cfloat_vector() {
        let x: Vec<CFLOAT> = cfloat_vec![1.0 + 2.0 * I, 3.0 - 4.0 * I, 5.0, 6.0 * I, -7.0 * I];

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
    fn cfloat_math() {
        let x: CFLOAT = cfloat!(346.0 - 78.0 * I);
        let y: CFLOAT = cfloat!(1.0 - 43.0 * I);

        // Add
        let z: CFLOAT = x + y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // AddAssign
        let mut z: CFLOAT = cfloat!(346.0 - 78.0 * I);
        z += y;
        assert_eq!(z.real(), 347.0);
        assert_eq!(z.imag(), -121.0);

        // Sub
        let z: CFLOAT = x - y;
        assert_eq!(z.real(), 345.0);
        assert_eq!(z.imag(), -35.0);

        // Mul
        let z: CFLOAT = x * y;
        assert_eq!(z.real(), -3008.0);
        assert_eq!(z.imag(), -14956.0);

        // Div
        let z: CFLOAT = x / y;
        assert_eq!(z.real(), 2.0);
        assert_eq!(z.imag(), 8.0);
    }
}
