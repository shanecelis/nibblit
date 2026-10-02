# bitlit

This crate checks equality bitmap literals and hexmap literals. 

## Motivation

When an algorithm produces visual results, its best to verify the results
visually even if its only a bitmap.

## The Trick

This crate was born out of a great bit (pun intended) of Rust syntax. Using bit
literal syntax `0b01`, specifying an inline bitmap is easy. Since `[]`
implements `Eq`, one can do the following without any crates:

```rust
# use bitlit::plot_bits;

#[rustfmt::skip]
assert_eq!(plot_bits::<u8, 8>((0..=7).map(|i| (i, i))), [
    0b10000000, // #.......
    0b01000000, // .#......
    0b00100000, // ..#.....
    0b00010000, // ...#....
    0b00001000, // ....#...
    0b00000100, // .....#..
    0b00000010, // ......#.
    0b00000001, // .......#
]);
```

## The Advantage 

The advantage this crate offers is what it prints when things don't match.

```rust
# use bitlit::{assert_bitmap_eq, plot_bits};

#[rustfmt::skip]
assert_bitmap_eq!(plot_bits::<u8, 8>((0..=7).map(|i| (i, i))), [
    0b10000000, // #.......
    0b01000000, // .#......
    0b00100000, // ..#.....
    0b00010000, // ...#....
    0b00001000, // ....#...
    0b00000100, // .....#..
    0b00000010, // ......#.
    0b00000001, // .......#
]);
```

Same line, but the plot omits `(0, 0)` and `(7, 7)` and adds the other two corners.
The assertion panics with an overlay (`#` match, `.` empty, `-` missing, `+` extra):

```rust,ignore
use bitlit::{assert_bitmap_eq, plot_bits};

#[rustfmt::skip]
assert_bitmap_eq!(
    plot_bits::<u8, 8>((1..=6).map(|i| (i, i)).chain([(7, 0), (0, 7)])),
    [
        0b10000000, // #.......
        0b01000000, // .#......
        0b00100000, // ..#.....
        0b00010000, // ...#....
        0b00001000, // ....#...
        0b00000100, // .....#..
        0b00000010, // ......#.
        0b00000001, // .......#
    ],
);
```

```text
bitmap mismatch (# match  . empty  - missing  + extra)
0 | -......+
1 | .#......
2 | ..#.....
3 | ...#....
4 | ....#...
5 | .....#..
6 | ......#.
7 | +......-
missing: (0, 0), (7, 7)
extra: (7, 0), (0, 7)
```

Bitmap overlay: `#` match, `.` empty, `-` missing, `+` extra. Runtime equality is
`==`. `Diff::bitmap` / `Diff::hexmap` return a `Diff` that formats as the overlay, without panicking.
`assert_bitmap_eq!` / `assert_hexmap_eq!` take an optional format string, same as
`assert_eq!`.

## Hexmap

Same diagonal, each cell a nibble. `u32` is eight nibbles wide. Values count by
two so `A`–`F` show up:

```rust
use bitlit::{assert_hexmap_eq, plot_hex};

#[rustfmt::skip]
assert_hexmap_eq!(plot_hex::<u32, 8>((0..=7).map(|i| (i, i, (2 * i + 1) as u8))), [
    0x10000000, // 1.......
    0x03000000, // .3......
    0x00500000, // ..5.....
    0x00070000, // ...7....
    0x00009000, // ....9...
    0x00000B00, // .....B..
    0x000000D0, // ......D.
    0x0000000F, // .......F
]);
```

Same line, but the plot omits `(0, 0)` and `(7, 7)` and adds the other two corners.
The assertion panics with an overlay (`.` empty, `1`–`F` match, `-` missing, `+`
extra, `*` changed):

```rust,ignore
use bitlit::{assert_hexmap_eq, plot_hex};

#[rustfmt::skip]
assert_hexmap_eq!(
    plot_hex::<u32, 8>(
        (1..=6)
            .map(|i| (i, i, (2 * i + 1) as u8))
            .chain([(7, 0, 1), (0, 7, 0xF)]),
    ),
    [
        0x10000000, // 1.......
        0x03000000, // .3......
        0x00500000, // ..5.....
        0x00070000, // ...7....
        0x00009000, // ....9...
        0x00000B00, // .....B..
        0x000000D0, // ......D.
        0x0000000F, // .......F
    ],
);
```

```text
hexmap mismatch (. empty  1-F match  - missing  + extra  * changed)
0 | -......+
1 | .3......
2 | ..5.....
3 | ...7....
4 | ....9...
5 | .....B..
6 | ......D.
7 | +......-
missing: (0, 0) 1, (7, 7) F
extra: (7, 0) 1, (0, 7) F
changed: none
```

Hex overlay prints the nibble on a match (`0` as `.`) and `*` when both cells are
set but differ.

## License

MIT
