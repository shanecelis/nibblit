//! 4-bit cells packed into integer rows (`0x…`). High nibble = x = 0.

use crate::{Diff, Packed};

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

/// Compare packed hexmap rows and panic with an overlay on mismatch.
#[doc(hidden)]
#[track_caller]
pub fn assert_hexmaps<T: Packed, const H: usize>(left: &[T; H], right: &[T; H]) {
    if let Some(diff) = Diff::hexmap(left, right) {
        panic!("hexmap mismatch (. empty  1-F match  - right only  + left only  * changed)\n{diff}");
    }
}

/// [`assert_hexmaps`] with a custom panic prefix, from `assert_hexmap_eq!(.., "{msg}", ...)`.
#[doc(hidden)]
#[track_caller]
pub fn assert_hexmaps_msg<T: Packed, const H: usize>(
    left: &[T; H],
    right: &[T; H],
    msg: core::fmt::Arguments<'_>,
) {
    if let Some(diff) = Diff::hexmap(left, right) {
        panic!(
            "hexmap mismatch (. empty  1-F match  - right only  + left only  * changed): {msg}\n{diff}"
        );
    }
}
