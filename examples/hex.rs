use bitlit::{assert_hexmap_eq, plot_hex};

fn main() {
    let err = std::panic::catch_unwind(|| {
        #[rustfmt::skip]
        assert_hexmap_eq!(
            plot_hex::<u32, 8>(
                (0..=7)
                    .map(|i| (i, i, (2 * i + 1) as u8))
            ),
            [
                0x10000000, // 1.......
                0x02000000, // .2......
                0x00500000, // ..5.....
                0x00070000, // ...7....
                0x00009000, // ....9...
                0x00000B00, // .....B..
                0x000000D0, // ......D.
                0x0000000F, // .......F
            ],
        );
    })
    .expect_err("should mismatch");
}
