// ===========================================================================
// useful traits
// ===========================================================================

pub trait BLASAbsoluteValue {
    type Output: PartialOrd;
    fn blas_abs(self) -> Self::Output;
}

pub trait Conjugatable {
    fn conj(self) -> Self;
}

pub trait Zeroable {
    fn zero() -> Self;
}
