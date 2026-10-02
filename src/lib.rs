#![doc = include_str!("../README.md")]

mod bitmap;
mod diff;
mod hexmap;
mod row;

pub use bitmap::plot_bits;
#[doc(hidden)]
pub use bitmap::{assert_bitmaps, assert_bitmaps_msg};
pub use diff::{Cell, Conflict, Diff};
pub use hexmap::plot_hex;
#[doc(hidden)]
pub use hexmap::{assert_hexmaps, assert_hexmaps_msg};
#[doc(hidden)]
pub use row::Packed;

/// Compare packed `0b` bitmap rows. Panics with an overlay on mismatch.
///
/// Overlay glyphs: `#` match, `.` empty, `-` right only, `+` left only.
/// Put `#[rustfmt::skip]` on the assertion so each row stays on its own line.
/// A trailing format string is included in the panic, same as [`assert_eq!`].
#[macro_export]
macro_rules! assert_bitmap_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::assert_bitmaps(&$actual, &$expected)
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        $crate::assert_bitmaps_msg(&$actual, &$expected, ::core::format_args!($($arg)+))
    };
}

/// Compare packed `0x` hexmap rows. Panics with an overlay on mismatch.
///
/// Overlay glyphs: `0` as `.`, `1`–`F` match, `-` right only, `+` left only, `*`
/// changed.
/// Put `#[rustfmt::skip]` on the assertion so each row stays on its own line.
/// A trailing format string is included in the panic, same as [`assert_eq!`].
#[macro_export]
macro_rules! assert_hexmap_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::assert_hexmaps(&$actual, &$expected)
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        $crate::assert_hexmaps_msg(&$actual, &$expected, ::core::format_args!($($arg)+))
    };
}
