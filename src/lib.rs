#![doc = include_str!("../README.md")]

mod bitmap;
mod hexmap;
mod row;

pub use bitmap::{assert_bitmaps, bitmap_diff, bitmap_eq, plot_bits, plot_spans};
pub use hexmap::{assert_hexmaps, hexmap_diff, hexmap_eq, plot_hex};
pub use row::Packed;

/// A cell-wise mismatch overlay.
///
/// `changed` is used by hexmaps when both cells are set but the nibbles
/// differ. Bitmaps leave it empty.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diff {
    pub overlay: String,
    pub missing: Vec<(u32, u32)>,
    pub extra: Vec<(u32, u32)>,
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
#[macro_export]
macro_rules! assert_bitmap_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::assert_bitmaps($actual, $expected)
    };
}

/// Compare packed `0x` hexmap rows. Panics with an overlay on mismatch.
///
/// Overlay glyphs: `0` as `.`, `1`–`F` match, `-` missing, `+` extra, `*`
/// changed.
/// Put `#[rustfmt::skip]` on the assertion so each row stays on its own line.
#[macro_export]
macro_rules! assert_hexmap_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::assert_hexmaps($actual, $expected)
    };
}

pub(crate) fn rows_eq<T: Packed>(actual: &[T], expected: &[T]) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected)
            .all(|(a, e)| a.bits() == e.bits())
}

pub(crate) fn fmt_points(points: &[(u32, u32)]) -> String {
    if points.is_empty() {
        return "none".into();
    }
    points
        .iter()
        .map(|(x, y)| format!("({x}, {y})"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn label_width(rows: usize) -> usize {
    rows.saturating_sub(1).to_string().len().max(1)
}
