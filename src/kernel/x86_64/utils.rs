// ===========================================================================
// x86_64 block sizes
// ===========================================================================
#[allow(unused)]
pub trait BlockSizes
where
    Self: Sized,
{
    const SSE_BLOCK_SIZE: usize;
    const AVX_BLOCK_SIZE: usize;
    const AVX2_BLOCK_SIZE: usize;
    const AVX512_BLOCK_SIZE: usize;
}

impl<T> BlockSizes for T {
    const SSE_BLOCK_SIZE: usize = 16 / size_of::<Self>();
    const AVX_BLOCK_SIZE: usize = 32 / size_of::<Self>();
    const AVX2_BLOCK_SIZE: usize = 32 / size_of::<Self>();
    const AVX512_BLOCK_SIZE: usize = 64 / size_of::<Self>();
}
