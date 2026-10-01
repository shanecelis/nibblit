# bitlit

Packed grid literals for tests. Rows are integers: `0b` for 1-bit cells, `0x`
for 4-bit cells. The high bit or nibble is `x = 0`. Width is in cells so
leading empty columns stay visible.

```rust
use bitlit::{assert_bitmap_eq, assert_hexmap_eq, bitmap_eq, plot_bits};

assert_bitmap_eq!([0b1010u8, 0b0101], [0b1010u8, 0b0101], 4);
assert!(bitmap_eq([0b1010u8], [0b1010u8], 4));
assert_eq!(plot_bits::<2>([(0, 0), (2, 1)], 3), [0b100, 0b001]);

assert_hexmap_eq!([0x1F0u16], [0x1F0u16], 3);
```

Bitmap overlay: `#` match, `.` empty, `-` missing, `+` extra. Hex overlay
prints the nibble on a match (`0` as `.`) and `*` when both cells are set but
differ. `bitmap_eq` / `hexmap_eq` return `bool`; `bitmap_diff` / `hexmap_diff`
return the overlay without panicking.

## License

MIT
