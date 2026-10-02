use nibblit::{assert_bitmap_eq, plot_bits};

fn main() {
    let err = std::panic::catch_unwind(|| {
        #[rustfmt::skip]
        assert_bitmap_eq!(
            plot_bits::<u8, 8>((1..=6).map(|i| (i, i)).chain([(0, 7)])),
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
    })
    .expect_err("should mismatch");
}
