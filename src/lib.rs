#![doc = include_str!("../README.md")]

mod bitmap;
mod hexmap;
mod row;

pub use bitmap::{assert_bitmaps, bitmap_diff, bitmap_eq, plot_bits, plot_spans};
pub use hexmap::{assert_hexmaps, hexmap_diff, hexmap_eq, plot_hex};
pub use row::Row;

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
#[macro_export]
macro_rules! assert_bitmap_eq {
    ($actual:expr, $expected:expr, $width:expr $(,)?) => {
        $crate::assert_bitmaps($actual, $expected, $width)
    };
}

/// Compare packed `0x` hexmap rows. Panics with an overlay on mismatch.
///
/// Overlay glyphs: `0` as `.`, `1`–`F` match, `-` missing, `+` extra, `*`
/// changed.
#[macro_export]
macro_rules! assert_hexmap_eq {
    ($actual:expr, $expected:expr, $width:expr $(,)?) => {
        $crate::assert_hexmaps($actual, $expected, $width)
    };
}

pub(crate) fn bit_mask(bits: u32) -> u64 {
    assert!(bits <= 64, "width {bits} exceeds 64 packed bits");
    if bits == 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    }
}

pub(crate) fn rows_eq<A, E>(actual: &[A], expected: &[E], bits: u32) -> bool
where
    A: Row,
    E: Row,
{
    if actual.len() != expected.len() {
        return false;
    }
    let mask = bit_mask(bits);
    actual
        .iter()
        .zip(expected)
        .all(|(a, e)| a.row() & mask == e.row() & mask)
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
