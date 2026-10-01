//! Integer rows that pack into a `u64`.

/// A packed row. Signed values keep their bit pattern (`as u64`).
pub trait Row: Copy {
    fn row(self) -> u64;
}

macro_rules! impl_row {
    ($($t:ty),*) => {$(
        impl Row for $t {
            #[inline]
            fn row(self) -> u64 {
                self as u64
            }
        }
    )*};
}

impl_row!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
