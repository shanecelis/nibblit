#![doc = include_str!("../README.md")]

mod bitmap;
mod hexmap;
mod row;

pub use bitmap::{bitmap_diff, plot_bits};
pub use hexmap::{hexmap_diff, plot_hex};
#[doc(hidden)]
pub use bitmap::{assert_bitmaps, assert_bitmaps_msg};
#[doc(hidden)]
pub use hexmap::{assert_hexmaps, assert_hexmaps_msg};
pub use row::Packed;

/// A cell-wise mismatch overlay.
///
/// `missing` / `extra` are `(x, y, cell)`. Bitmaps store `1` in `cell`; hexmaps
/// store the nibble. `changed` is hexmaps only: `(x, y, actual, expected)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diff {
    pub overlay: String,
    pub missing: Vec<(u32, u32, u8)>,
    pub extra: Vec<(u32, u32, u8)>,
    pub changed: Vec<(u32, u32, u8, u8)>,
}

impl core::fmt::Display for Diff {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.overlay)
    }
}

/// Compare packed `0b` bitmap rows. Panics with an overlay on mismatch.
///
/// Overlay glyphs: `#` match, `.` empty, `-` missing, `+` extra.
/// Put `#[rustfmt::skip]` on the assertion so each row stays on its own line.
/// A trailing format string is included in the panic, same as [`assert_eq!`].
#[macro_export]
macro_rules! assert_bitmap_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::assert_bitmaps($actual, $expected)
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        $crate::assert_bitmaps_msg($actual, $expected, ::core::format_args!($($arg)+))
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
        $crate::assert_hexmaps($actual, $expected)
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        $crate::assert_hexmaps_msg($actual, $expected, ::core::format_args!($($arg)+))
    };
}

pub(crate) fn fmt_points(points: &[(u32, u32, u8)]) -> String {
    if points.is_empty() {
        return "none".into();
    }
    points
        .iter()
        .map(|(x, y, _)| format!("({x}, {y})"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn fmt_cells(cells: &[(u32, u32, u8)]) -> String {
    if cells.is_empty() {
        return "none".into();
    }
    cells
        .iter()
        .map(|(x, y, n)| format!("({x}, {y}) {n:X}"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn label_width(rows: usize) -> usize {
    rows.saturating_sub(1).to_string().len().max(1)
}
