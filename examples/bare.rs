use bitlit::{plot_bits, plot_hex};
use std::panic;

fn example(f: impl FnOnce() + panic::UnwindSafe) {
    let _ = panic::catch_unwind(f);
}

fn bare_bits() {
    #[rustfmt::skip]
    assert_eq!(plot_bits::<u8, 8>((1..=7).map(|i| (i, i))), [
        0b10000000, // #.......
        0b01000000, // .#......
        0b00100000, // ..#.....
        0b00010000, // ...#....
        0b00001000, // ....#...
        0b00000100, // .....#..
        0b00000010, // ......#.
        0b00000001, // .......#
    ]);
}

fn bare_hex() {
    #[rustfmt::skip]
    assert_eq!(plot_hex::<u32, 8>((0..=7).map(|i| (i, i, (2 * i + 1) as u8))), [
        0x10000000, // 1.......
        0x04000000, // .4......
        0x00500000, // ..5.....
        0x00070000, // ...7....
        0x00009000, // ....9...
        0x00000B00, // .....B..
        0x000000D0, // ......D.
        0x0000000F, // .......F
    ]);
}

fn main() {
    example(bare_bits);
    example(bare_hex);
}
