//! Integer row types whose width is `size_of::<T>() * 8`.

/// A packed row. Width is `Self::BITS` cells (1-bit) or nibbles (`BITS / 4`).
pub trait Packed: Copy + Eq {
    const BITS: u32;

    fn from_bits(bits: u64) -> Self;
    fn bits(self) -> u64;
}

macro_rules! impl_packed {
    ($($t:ty),*) => {$(
        impl Packed for $t {
            const BITS: u32 = <$t>::BITS;

            #[inline]
            fn from_bits(bits: u64) -> Self {
                bits as $t
            }

            #[inline]
            fn bits(self) -> u64 {
                self as u64
            }
        }
    )*};
}

impl_packed!(u8, u16, u32, u64);
