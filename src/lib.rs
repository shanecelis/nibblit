#![doc = include_str!("../README.md")]

mod bitmap;
mod diff;
mod hexmap;
mod row;

#[doc(hidden)]
pub use bitmap::{assert_bitmaps, assert_bitmaps_msg};
pub use bitmap::{bitmap_diff, plot_bits};
pub use diff::{Diff, DiffStats};
#[doc(hidden)]
pub use hexmap::{assert_hexmaps, assert_hexmaps_msg};
pub use hexmap::{hexmap_diff, plot_hex};
pub use row::Packed;

/// Compare packed `0b` bitmap rows. Panics with an overlay on mismatch.
///
/// Overlay glyphs: `#` match, `.` empty, `-` missing, `+` extra.
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
/// Overlay glyphs: `0` as `.`, `1`–`F` match, `-` missing, `+` extra, `*`
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
