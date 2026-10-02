# bitlit

This crate checks equality for bitmap literals and hexmap literals.

## Motivation

When an algorithm produces visual results, its best to verify the results
visually even if its only a bitmap.

## The Trick

This crate was born out of a great bit (pun intended) of Rust syntax. Using bit
literal syntax `0b01`, specifying an inline bitmap is easy. Since `[]`
implements `Eq`, one can do the following without any crate:

```rust,should_panic
# use bitlit::plot_bits;
#[rustfmt::skip]
assert_eq!(plot_bits::<u8, 8>((1..=6).map(|i| (i, i)).chain([(0,7])), [
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
## The Problem

The problem comes when the assertion fails, and one has to decipher how the
bit literals are represented in decimal.

```text
assertion `left == right` failed
  left: [0, 64, 32, 16, 8, 4, 2, 128]
 right: [128, 64, 32, 16, 8, 4, 2, 1]
```

## The Advantage 

The beauty of this crate is what if offers when things do not match: an ASCII
representation of the bitmap along with details of where the bits were or
weren't.

```text
bitmap mismatch (# match  . empty  - right only  + left only)
0 | -.......
1 | .#......
2 | ..#.....
3 | ...#....
4 | ....#...
5 | .....#..
6 | ......#.
7 | +......-
left only: (0, 7)
right only: (0, 0), (7, 7)
```

# Hexmap

Bitmaps suffice in many cases but when one needs more than 1 bit, hexadecimal
offers 16 values per cell in each half byte or nybble [0, F].

```rust,should_panic
# use bitlit::plot_hex;
#[rustfmt::skip]
assert_eq!(plot_hex::<u32, 8>((0..=7).map(|i| (i, i, (2 * i + 1) as u8))), [
    0x10000000, // 1.......
    0x02000000, // .2......
    0x00500000, // ..5.....
    0x00070000, // ...7....
    0x00009000, // ....9...
    0x00000B00, // .....B..
    0x000000D0, // ......D.
    0x0000000F, // .......F
]);
```

While literal bitmaps can be deciphered without a crate such as this, I believe
trying to decipher a hexmap unassisted is folly. Not convinced? Try to find the
difference with the assertion failure below.

``` text
assertion `left == right` failed
  left: [268435456, 50331648, 5242880, 458752, 36864, 2816, 208, 15]
 right: [268435456, 33554432, 5242880, 458752, 36864, 2816, 208, 15]
```

No shame in not being a hexadecimally gifted machine. Try to determine the
difference using bitlit below.

```text
hexmap mismatch (. empty  1-F match  - right only  + left only  * changed)
0 | 1.......
1 | .*......
2 | ..5.....
3 | ...7....
4 | ....9...
5 | .....B..
6 | ......D.
7 | .......F
left only: none
right only: none
changed: (1, 1) 3≠2
```

## Questions

### Can bitmaps be wider than 8 bits?

Yes, bitmaps can be 8, 16, 32, or 64 bits wide. Just use the corresponding data
type u8, u16, u32, or u64.

## License

This crate is licensed under the MIT License or the Apache License 2.0.
