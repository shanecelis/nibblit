use bitlit::{assert_hexmap_eq, plot_hex};

fn main() {
    let err = std::panic::catch_unwind(|| {
        #[rustfmt::skip]
        assert_hexmap_eq!(
            plot_hex::<u32, 8>(
                (1..=6)
                    .map(|i| (i, i, (2 * i + 1) as u8))
                    .chain([(7, 0, 1), (0, 7, 0xF)]),
            ),
            [
                0x10000000, // 1.......
                0x02000000, // .3......
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
    println!("{}", err.downcast_ref::<String>().unwrap());
}
