# bitlit

Packed grid literals for tests. Rows are integers: `0b` for 1-bit cells, `0x`
for 4-bit cells. The high bit or nibble is `x = 0`. Width is the row type:
`u8` is 8 cells, `u16` is 16, and so on.

Write one row per line and put `#[rustfmt::skip]` on the assertion so rustfmt
does not wrap the grid.

```rust
use bitlit::{assert_bitmap_eq, assert_hexmap_eq, plot_bits};

#[rustfmt::skip]
assert_bitmap_eq!(
    [
        0b1010_0000u8,
        0b0101_0000,
    ],
    [
        0b1010_0000,
        0b0101_0000,
    ],
);
assert_eq!(
    plot_bits::<u8, 2>([(0, 0), (2, 1)]),
    [0b1000_0000, 0b0010_0000],
);

#[rustfmt::skip]
assert_hexmap_eq!([0x1F00u16], [0x1F00]);
```

Bitmap overlay: `#` match, `.` empty, `-` missing, `+` extra. Hex overlay
prints the nibble on a match (`0` as `.`) and `*` when both cells are set but
differ. Runtime equality is `==`. `bitmap_diff` / `hexmap_diff` return the
overlay without panicking.

## License

MIT
