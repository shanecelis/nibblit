//! 1-bit cells packed into integer rows (`0b…`). MSB = x = 0.

use crate::{Diff, Packed};

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

/// Compare packed bitmap rows and panic with an overlay on mismatch.
#[doc(hidden)]
#[track_caller]
pub fn assert_bitmaps<T: Packed, const H: usize>(left: &[T; H], right: &[T; H]) {
    if let Some(diff) = Diff::bitmap(left, right) {
        panic!("bitmap mismatch (# match  . empty  - missing  + extra)\n{diff}");
    }
}

/// [`assert_bitmaps`] with a custom panic prefix, from `assert_bitmap_eq!(.., "{msg}", ...)`.
#[doc(hidden)]
#[track_caller]
pub fn assert_bitmaps_msg<T: Packed, const H: usize>(
    left: &[T; H],
    right: &[T; H],
    msg: core::fmt::Arguments<'_>,
) {
    if let Some(diff) = Diff::bitmap(left, right) {
        panic!("bitmap mismatch (# match  . empty  - missing  + extra): {msg}\n{diff}");
    }
}
