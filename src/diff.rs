//! Borrowed grids and the overlay produced by walking them.

use crate::Packed;
use core::fmt::{self, Write};

/// A mismatch between two packed grids of height `H`.
///
/// Left is the first grid, right is the second. Formatting writes the overlay;
/// [`Self::conflicts`] reports the cells behind it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Diff<'a, T: Packed, const H: usize> {
    left: &'a [T; H],
    right: &'a [T; H],
    mode: Mode,
}

/// How a cell differs between the two grids.
///
/// The `u8` is `1` for a bitmap and the nibble for a hexmap. [`Conflict::Changed`]
/// is hexmaps only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conflict {
    /// On the left; absent from the right.
    Left(u8),
    /// On the right; absent from the left.
    Right(u8),
    /// Set on both sides, with different values.
    Changed { left: u8, right: u8 },
}

/// A differing cell and where it sits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub conflict: Conflict,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Bitmap,
    Hexmap,
}

impl<'a, T: Packed, const H: usize> Diff<'a, T, H> {
    /// Bitmap mismatch, or `None` when the grids match.
    ///
    /// Borrows both grids. Formatting writes the overlay (`#` match, `.` empty,
    /// `-` right only, `+` left only).
    pub fn bitmap(left: &'a [T; H], right: &'a [T; H]) -> Option<Self> {
        if left == right {
            None
        } else {
            Some(Self {
                left,
                right,
                mode: Mode::Bitmap,
            })
        }
    }

    /// Hexmap mismatch, or `None` when the grids match.
    ///
    /// Borrows both grids. Formatting writes the overlay (`.` empty, `1`–`F`
    /// match, `-` right only, `+` left only, `*` changed).
    pub fn hexmap(left: &'a [T; H], right: &'a [T; H]) -> Option<Self> {
        if left == right {
            None
        } else {
            Some(Self {
                left,
                right,
                mode: Mode::Hexmap,
            })
        }
    }

    /// Cells that differ, in row-major order.
    pub fn conflicts(&self) -> impl Iterator<Item = Cell> + '_ {
        let width = self.width();
        (0..H).flat_map(move |y| {
            (0..width).filter_map(move |x| {
                self.classify(x, y).1.map(|conflict| Cell {
                    x,
                    y: y as u32,
                    conflict,
                })
            })
        })
    }

    fn width(&self) -> u32 {
        match self.mode {
            Mode::Bitmap => T::BITS,
            Mode::Hexmap => T::BITS / 4,
        }
    }

    fn classify(&self, x: u32, y: usize) -> (char, Option<Conflict>) {
        let left_bits = self.left[y].bits();
        let right_bits = self.right[y].bits();
        match self.mode {
            Mode::Bitmap => {
                let shift = T::BITS - 1 - x;
                let left = (left_bits >> shift) & 1 != 0;
                let right = (right_bits >> shift) & 1 != 0;
                match (left, right) {
                    (true, true) => ('#', None),
                    (false, false) => ('.', None),
                    (false, true) => ('-', Some(Conflict::Right(1))),
                    (true, false) => ('+', Some(Conflict::Left(1))),
                }
            }
            Mode::Hexmap => {
                let shift = 4 * (self.width() - 1 - x);
                let left = ((left_bits >> shift) & 0xf) as u8;
                let right = ((right_bits >> shift) & 0xf) as u8;
                match (left, right) {
                    (0, 0) => ('.', None),
                    (left, right) if left == right => (
                        char::from_digit(u32::from(left), 16)
                            .unwrap()
                            .to_ascii_uppercase(),
                        None,
                    ),
                    (0, value) => ('-', Some(Conflict::Right(value))),
                    (value, 0) => ('+', Some(Conflict::Left(value))),
                    (left, right) => ('*', Some(Conflict::Changed { left, right })),
                }
            }
        }
    }

    fn write_footer(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            Mode::Bitmap => {
                f.write_str("left only: ")?;
                write_separated(
                    f,
                    self.conflicts().filter_map(|item| match item.conflict {
                        Conflict::Left(_) => Some((item.x, item.y)),
                        _ => None,
                    }),
                    |f, (x, y)| write!(f, "({x}, {y})"),
                )?;
                f.write_char('\n')?;
                f.write_str("right only: ")?;
                write_separated(
                    f,
                    self.conflicts().filter_map(|item| match item.conflict {
                        Conflict::Right(_) => Some((item.x, item.y)),
                        _ => None,
                    }),
                    |f, (x, y)| write!(f, "({x}, {y})"),
                )
            }
            Mode::Hexmap => {
                f.write_str("left only: ")?;
                write_separated(
                    f,
                    self.conflicts().filter_map(|item| match item.conflict {
                        Conflict::Left(value) => Some((item.x, item.y, value)),
                        _ => None,
                    }),
                    |f, (x, y, cell)| write!(f, "({x}, {y}) {cell:X}"),
                )?;
                f.write_char('\n')?;
                f.write_str("right only: ")?;
                write_separated(
                    f,
                    self.conflicts().filter_map(|item| match item.conflict {
                        Conflict::Right(value) => Some((item.x, item.y, value)),
                        _ => None,
                    }),
                    |f, (x, y, cell)| write!(f, "({x}, {y}) {cell:X}"),
                )?;
                f.write_char('\n')?;
                f.write_str("changed: ")?;
                write_separated(
                    f,
                    self.conflicts().filter_map(|item| match item.conflict {
                        Conflict::Changed { left, right } => Some((item.x, item.y, left, right)),
                        _ => None,
                    }),
                    |f, (x, y, left, right)| write!(f, "({x}, {y}) {left:X}≠{right:X}"),
                )
            }
        }
    }
}

impl<T: Packed, const H: usize> fmt::Display for Diff<'_, T, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = label_width(H);
        let width = self.width();
        for y in 0..H {
            write!(f, "{y:label$} | ")?;
            for x in 0..width {
                f.write_char(self.classify(x, y).0)?;
            }
            f.write_char('\n')?;
        }
        self.write_footer(f)
    }
}

fn write_separated<T>(
    f: &mut fmt::Formatter<'_>,
    items: impl Iterator<Item = T>,
    mut write_item: impl FnMut(&mut fmt::Formatter<'_>, T) -> fmt::Result,
) -> fmt::Result {
    let mut wrote = false;
    for item in items {
        if wrote {
            f.write_str(", ")?;
        }
        write_item(f, item)?;
        wrote = true;
    }
    if !wrote {
        f.write_str("none")?;
    }
    Ok(())
}

fn label_width(rows: usize) -> usize {
    rows.saturating_sub(1).to_string().len().max(1)
}
