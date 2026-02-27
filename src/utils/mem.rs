// ===========================================================================
// memory alignment
// ===========================================================================

/// Checks that the given vector is aligned to the specified alignment.
///
/// # Arguments
///
/// * `v` - a vector
///
/// * `alignment` - desired alignment in bits
pub fn is_aligned<T>(v: &[T], alignment: usize) -> bool {
    let ptr = v.as_ptr() as usize;
    ptr % alignment == 0
}
