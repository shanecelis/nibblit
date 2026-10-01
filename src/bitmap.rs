//! 1-bit cells packed into integer rows (`0b…`). MSB = x = 0.

use crate::{fmt_points, label_width, rows_eq, Diff, Row};
use std::fmt::Write;

/// Pack points into `H` rows of `width` bits, MSB = x = 0.
#[track_caller]
pub fn plot_bits<const H: usize>(
    points: impl IntoIterator<Item = (isize, isize)>,
    width: u32,
) -> [u64; H] {
    assert!(width <= 64, "bitmap width {width} exceeds 64 bits");
    let mut grid = [0u64; H];
    for (x, y) in points {
        assert!(
            (0..width as isize).contains(&x) && (0..H as isize).contains(&y),
            "({x},{y}) off {width}x{H}"
        );
        grid[y as usize] |= 1 << (width - 1 - x as u32);
    }
    grid
}

/// Pack inclusive `[x0, x1]` spans on row `y`, MSB = x = 0.
#[track_caller]
pub fn plot_spans<const H: usize>(
    spans: impl IntoIterator<Item = (isize, isize, isize)>,
    width: u32,
) -> [u64; H] {
    plot_bits(
        spans.into_iter().flat_map(|(x0, x1, y)| {
            assert!(x0 <= x1, "x0={x0} > x1={x1} y={y}");
            (x0..=x1).map(move |x| (x, y))
        }),
        width,
    )
}

/// `true` when the visible `width` bits of each row match.
pub fn bitmap_eq<A, E, Ta, Te>(actual: A, expected: E, width: u32) -> bool
where
    A: AsRef<[Ta]>,
    E: AsRef<[Te]>,
    Ta: Row,
    Te: Row,
{
    rows_eq(actual.as_ref(), expected.as_ref(), width)
}

/// Overlay of a mismatch, or `None` when the grids match.
pub fn bitmap_diff<A, E, Ta, Te>(actual: A, expected: E, width: u32) -> Option<Diff>
where
    A: AsRef<[Ta]>,
    E: AsRef<[Te]>,
    Ta: Row,
    Te: Row,
{
    if bitmap_eq(&actual, &expected, width) {
        return None;
    }
    Some(overlay(actual.as_ref(), expected.as_ref(), width))
}

/// Compare packed bitmap rows and panic with an overlay on mismatch.
#[track_caller]
pub fn assert_bitmaps<A, E, Ta, Te>(actual: A, expected: E, width: u32)
where
    A: AsRef<[Ta]>,
    E: AsRef<[Te]>,
    Ta: Row,
    Te: Row,
{
    if let Some(diff) = bitmap_diff(actual, expected, width) {
        panic!("bitmap mismatch (# match  . empty  - missing  + extra)\n{diff}");
    }
}

fn overlay<A, E>(actual: &[A], expected: &[E], width: u32) -> Diff
where
    A: Row,
    E: Row,
{
    let rows = actual.len().max(expected.len());
    let label = label_width(rows);
    let mut missing = Vec::new();
    let mut extra = Vec::new();
    let mut out = String::new();
    for y in 0..rows {
        let act = actual.get(y).copied().map(Row::row).unwrap_or(0);
        let exp = expected.get(y).copied().map(Row::row).unwrap_or(0);
        let _ = write!(out, "{y:label$} | ");
        for x in 0..width {
            let shift = width - 1 - x;
            let a = (act >> shift) & 1 != 0;
            let e = (exp >> shift) & 1 != 0;
            out.push(match (e, a) {
                (true, true) => '#',
                (false, false) => '.',
                (true, false) => {
                    missing.push((x, y as u32));
                    '-'
                }
                (false, true) => {
                    extra.push((x, y as u32));
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
