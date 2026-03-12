// ===========================================================================
// useful traits
// ===========================================================================

pub trait Zeroable {
    fn zero() -> Self;
}

pub trait BLASAbsoluteValue {
    type Output: PartialOrd;
    fn blas_abs(self) -> Self::Output;
}
