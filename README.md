# bitlit

Packed grid literals for tests. Rows are integers: `0b` for 1-bit cells, `0x`
for 4-bit cells. The high bit or nibble is `x = 0`. Width is the row type:
`u8` is 8 cells, `u16` is 16, and so on.

Write one row per line and put `#[rustfmt::skip]` on the assertion so rustfmt
does not wrap the grid.

```rust
use bitlit::{assert_bitmap_eq, assert_hexmap_eq, plot_bits};

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

#[rustfmt::skip]
assert_hexmap_eq!([0x1F00u16], [0x1F00]);
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

Bitmap overlay: `#` match, `.` empty, `-` missing, `+` extra. Hex overlay
prints the nibble on a match (`0` as `.`) and `*` when both cells are set but
differ. Runtime equality is `==`. `bitmap_diff` / `hexmap_diff` return the
overlay without panicking. `assert_bitmap_eq!` / `assert_hexmap_eq!` take an
optional format string, same as `assert_eq!`.

## License

MIT
