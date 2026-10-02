//! Owned grids and the overlay produced by walking them.

use crate::Packed;
use core::fmt::{self, Write};

/// A mismatch between two packed grids of height `H`.
///
/// Left is the first grid, right is the second. The rows stay in the arrays.
/// Formatting writes the overlay; [`Self::stats`] reports the cells behind it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Diff<T: Packed, const H: usize> {
    left: [T; H],
    right: [T; H],
    mode: Mode,
}

/// Cells that differ between the two grids.
///
/// `missing` / `extra` are `(x, y, cell)`. Bitmaps store `1` in `cell`; hexmaps
/// store the nibble. `changed` is hexmaps only: `(x, y, left, right)`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffStats {
    pub missing: Vec<(u32, u32, u8)>,
    pub extra: Vec<(u32, u32, u8)>,
    pub changed: Vec<(u32, u32, u8, u8)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Bitmap,
    Hexmap,
}

#[derive(Clone, Copy)]
enum Class {
    Same(char),
    Missing(u8),
    Extra(u8),
    Changed { left: u8, right: u8 },
}

impl Class {
    fn glyph(self) -> char {
        match self {
            Class::Same(glyph) => glyph,
            Class::Missing(_) => '-',
            Class::Extra(_) => '+',
            Class::Changed { .. } => '*',
        }
    }
}

impl<T: Packed, const H: usize> Diff<T, H> {
    pub(crate) fn bitmap(left: [T; H], right: [T; H]) -> Option<Self> {
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

    pub(crate) fn hexmap(left: [T; H], right: [T; H]) -> Option<Self> {
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

    /// Cells behind the overlay.
    pub fn stats(&self) -> DiffStats {
        let mut stats = DiffStats::default();
        let width = self.width();
        for y in 0..H {
            for x in 0..width {
                stats.record(x, y as u32, self.class_at(x, y));
            }
        }
        stats
    }

    fn width(&self) -> u32 {
        match self.mode {
            Mode::Bitmap => T::BITS,
            Mode::Hexmap => T::BITS / 4,
        }
    }

    fn class_at(&self, x: u32, y: usize) -> Class {
        let left_bits = self.left[y].bits();
        let right_bits = self.right[y].bits();
        match self.mode {
            Mode::Bitmap => {
                let shift = T::BITS - 1 - x;
                let left = (left_bits >> shift) & 1 != 0;
                let right = (right_bits >> shift) & 1 != 0;
                match (left, right) {
                    (true, true) => Class::Same('#'),
                    (false, false) => Class::Same('.'),
                    (false, true) => Class::Missing(1),
                    (true, false) => Class::Extra(1),
                }
            }
            Mode::Hexmap => {
                let shift = 4 * (self.width() - 1 - x);
                let left = ((left_bits >> shift) & 0xf) as u8;
                let right = ((right_bits >> shift) & 0xf) as u8;
                match (left, right) {
                    (0, 0) => Class::Same('.'),
                    (left, right) if left == right => Class::Same(
                        char::from_digit(u32::from(left), 16)
                            .unwrap()
                            .to_ascii_uppercase(),
                    ),
                    (0, right) => Class::Missing(right),
                    (left, 0) => Class::Extra(left),
                    (left, right) => Class::Changed { left, right },
                }
            }
        }
    }

    fn write_footer(&self, f: &mut fmt::Formatter<'_>, stats: &DiffStats) -> fmt::Result {
        match self.mode {
            Mode::Bitmap => {
                f.write_str("missing: ")?;
                write_points(f, &stats.missing)?;
                f.write_char('\n')?;
                f.write_str("extra: ")?;
                write_points(f, &stats.extra)
            }
            Mode::Hexmap => {
                f.write_str("missing: ")?;
                write_cells(f, &stats.missing)?;
                f.write_char('\n')?;
                f.write_str("extra: ")?;
                write_cells(f, &stats.extra)?;
                f.write_char('\n')?;
                f.write_str("changed: ")?;
                write_changed(f, &stats.changed)
            }
        }
    }
}

impl DiffStats {
    fn record(&mut self, x: u32, y: u32, class: Class) {
        match class {
            Class::Same(_) => {}
            Class::Missing(cell) => self.missing.push((x, y, cell)),
            Class::Extra(cell) => self.extra.push((x, y, cell)),
            Class::Changed { left, right } => self.changed.push((x, y, left, right)),
        }
    }
}

impl<T: Packed, const H: usize> fmt::Display for Diff<T, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stats = DiffStats::default();
        let label = label_width(H);
        let width = self.width();
        for y in 0..H {
            write!(f, "{y:label$} | ")?;
            for x in 0..width {
                let class = self.class_at(x, y);
                f.write_char(class.glyph())?;
                stats.record(x, y as u32, class);
            }
            f.write_char('\n')?;
        }
        self.write_footer(f, &stats)
    }
}

fn write_points(f: &mut fmt::Formatter<'_>, points: &[(u32, u32, u8)]) -> fmt::Result {
    write_separated(f, points, |f, (x, y, _)| write!(f, "({x}, {y})"))
}

fn write_cells(f: &mut fmt::Formatter<'_>, cells: &[(u32, u32, u8)]) -> fmt::Result {
    write_separated(f, cells, |f, (x, y, n)| write!(f, "({x}, {y}) {n:X}"))
}

fn write_changed(f: &mut fmt::Formatter<'_>, cells: &[(u32, u32, u8, u8)]) -> fmt::Result {
    write_separated(f, cells, |f, (x, y, left, right)| {
        write!(f, "({x}, {y}) {left:X}≠{right:X}")
    })
}

fn write_separated<T>(
    f: &mut fmt::Formatter<'_>,
    items: &[T],
    mut write_item: impl FnMut(&mut fmt::Formatter<'_>, &T) -> fmt::Result,
) -> fmt::Result {
    if items.is_empty() {
        return f.write_str("none");
    }
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            f.write_str(", ")?;
        }
        write_item(f, item)?;
    }
    Ok(())
}

fn label_width(rows: usize) -> usize {
    rows.saturating_sub(1).to_string().len().max(1)
}
