//! 4-bit cells packed into integer rows (`0x…`). High nibble = x = 0.

use crate::{fmt_points, label_width, Diff, Packed};
use std::fmt::Write;

/// Pack `(x, y, nibble)` cells into `H` rows of `T::BITS / 4` nibbles.
///
/// High nibble = x = 0. `nibble` must be `0..=15`.
#[track_caller]
pub fn plot_hex<T: Packed, const H: usize>(
    cells: impl IntoIterator<Item = (isize, isize, u8)>,
) -> [T; H] {
    let width = T::BITS / 4;
    let mut grid = [0u64; H];
    for (x, y, nibble) in cells {
        assert!(nibble <= 0xf, "nibble {nibble:#x} at ({x},{y}) exceeds 0xf");
        assert!(
            (0..width as isize).contains(&x) && (0..H as isize).contains(&y),
            "({x},{y}) off {width}x{H}"
        );
        grid[y as usize] |= u64::from(nibble) << (4 * (width - 1 - x as u32));
    }
    core::array::from_fn(|y| T::from_bits(grid[y]))
}

/// Overlay of a mismatch, or `None` when the grids match.
pub fn hexmap_diff<T: Packed>(actual: impl AsRef<[T]>, expected: impl AsRef<[T]>) -> Option<Diff> {
    if actual.as_ref() == expected.as_ref() {
        return None;
    }
    Some(overlay(actual.as_ref(), expected.as_ref()))
}

/// Compare packed hexmap rows and panic with an overlay on mismatch.
#[doc(hidden)]
#[track_caller]
pub fn assert_hexmaps<T: Packed>(actual: impl AsRef<[T]>, expected: impl AsRef<[T]>) {
    if let Some(diff) = hexmap_diff(actual, expected) {
        panic!("hexmap mismatch (. empty  1-F match  - missing  + extra  * changed)\n{diff}");
    }
}

/// [`assert_hexmaps`] with a custom panic prefix, from `assert_hexmap_eq!(.., "{msg}", ...)`.
#[doc(hidden)]
#[track_caller]
pub fn assert_hexmaps_msg<T: Packed>(
    actual: impl AsRef<[T]>,
    expected: impl AsRef<[T]>,
    msg: core::fmt::Arguments<'_>,
) {
    if let Some(diff) = hexmap_diff(actual, expected) {
        panic!(
            "hexmap mismatch (. empty  1-F match  - missing  + extra  * changed): {msg}\n{diff}"
        );
    }
}

fn overlay<T: Packed>(actual: &[T], expected: &[T]) -> Diff {
    let width = T::BITS / 4;
    let rows = actual.len().max(expected.len());
    let label = label_width(rows);
    let mut missing = Vec::new();
    let mut extra = Vec::new();
    let mut changed = Vec::new();
    let mut out = String::new();
    for y in 0..rows {
        let act = actual.get(y).copied().map(Packed::bits).unwrap_or(0);
        let exp = expected.get(y).copied().map(Packed::bits).unwrap_or(0);
        let _ = write!(out, "{y:label$} | ");
        for x in 0..width {
            let shift = 4 * (width - 1 - x);
            let a = ((act >> shift) & 0xf) as u8;
            let e = ((exp >> shift) & 0xf) as u8;
            out.push(match (e, a) {
                (0, 0) => '.',
                (e, a) if e == a => char::from_digit(e as u32, 16).unwrap().to_ascii_uppercase(),
                (_, 0) => {
                    missing.push((x, y as u32));
                    '-'
                }
                (0, _) => {
                    extra.push((x, y as u32));
                    '+'
                }
                _ => {
                    changed.push((x, y as u32, a, e));
                    '*'
                }
            });
        }
        out.push('\n');
    }
    let _ = writeln!(out, "missing: {}", fmt_points(&missing));
    let _ = writeln!(out, "extra: {}", fmt_points(&extra));
    let _ = write!(out, "changed: {}", fmt_changed(&changed));
    Diff {
        overlay: out,
        missing,
        extra,
        changed,
    }
}

fn fmt_changed(cells: &[(u32, u32, u8, u8)]) -> String {
    if cells.is_empty() {
        return "none".into();
    }
    cells
        .iter()
        .map(|(x, y, a, e)| format!("({x}, {y}) {a:X}≠{e:X}"))
        .collect::<Vec<_>>()
        .join(", ")
}
