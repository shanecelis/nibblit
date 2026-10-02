//! 1-bit cells packed into integer rows (`0b…`). MSB = x = 0.

use crate::{fmt_points, label_width, Diff, Packed};
use std::fmt::Write;

/// Pack points into `H` rows of `T::BITS` bits, MSB = x = 0.
#[track_caller]
pub fn plot_bits<T: Packed, const H: usize>(
    points: impl IntoIterator<Item = (isize, isize)>,
) -> [T; H] {
    let width = T::BITS;
    let mut grid = [0u64; H];
    for (x, y) in points {
        assert!(
            (0..width as isize).contains(&x) && (0..H as isize).contains(&y),
            "({x},{y}) off {width}x{H}"
        );
        grid[y as usize] |= 1 << (width - 1 - x as u32);
    }
    core::array::from_fn(|y| T::from_bits(grid[y]))
}

/// Overlay of a mismatch, or `None` when the grids match.
pub fn bitmap_diff<T: Packed>(actual: impl AsRef<[T]>, expected: impl AsRef<[T]>) -> Option<Diff> {
    if actual.as_ref() == expected.as_ref() {
        return None;
    }
    Some(overlay(actual.as_ref(), expected.as_ref()))
}

/// Compare packed bitmap rows and panic with an overlay on mismatch.
#[doc(hidden)]
#[track_caller]
pub fn assert_bitmaps<T: Packed>(actual: impl AsRef<[T]>, expected: impl AsRef<[T]>) {
    if let Some(diff) = bitmap_diff(actual, expected) {
        panic!("bitmap mismatch (# match  . empty  - missing  + extra)\n{diff}");
    }
}

/// [`assert_bitmaps`] with a custom panic prefix, from `assert_bitmap_eq!(.., "{msg}", ...)`.
#[doc(hidden)]
#[track_caller]
pub fn assert_bitmaps_msg<T: Packed>(
    actual: impl AsRef<[T]>,
    expected: impl AsRef<[T]>,
    msg: core::fmt::Arguments<'_>,
) {
    if let Some(diff) = bitmap_diff(actual, expected) {
        panic!("bitmap mismatch (# match  . empty  - missing  + extra): {msg}\n{diff}");
    }
}

fn overlay<T: Packed>(actual: &[T], expected: &[T]) -> Diff {
    let width = T::BITS;
    let rows = actual.len().max(expected.len());
    let label = label_width(rows);
    let mut missing = Vec::new();
    let mut extra = Vec::new();
    let mut out = String::new();
    for y in 0..rows {
        let act = actual.get(y).copied().map(Packed::bits).unwrap_or(0);
        let exp = expected.get(y).copied().map(Packed::bits).unwrap_or(0);
        let _ = write!(out, "{y:label$} | ");
        for x in 0..width {
            let shift = width - 1 - x;
            let a = (act >> shift) & 1 != 0;
            let e = (exp >> shift) & 1 != 0;
            out.push(match (e, a) {
                (true, true) => '#',
                (false, false) => '.',
                (true, false) => {
                    missing.push((x, y as u32, 1));
                    '-'
                }
                (false, true) => {
                    extra.push((x, y as u32, 1));
                    '+'
                }
            });
        }
        out.push('\n');
    }
    let _ = writeln!(out, "missing: {}", fmt_points(&missing));
    let _ = write!(out, "extra: {}", fmt_points(&extra));
    Diff {
        overlay: out,
        missing,
        extra,
        changed: Vec::new(),
    }
}
