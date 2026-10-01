use bitlit::{
    assert_bitmap_eq, assert_hexmap_eq, bitmap_diff, hexmap_diff, hexmap_eq, plot_bits, plot_hex,
    plot_spans,
};

#[test]
fn equal_grids_pass() {
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
}

#[test]
fn plot_bits_packs_msb_left() {
    assert_eq!(
        plot_bits::<u8, 2>([(0, 0), (2, 1)]),
        [0b1000_0000, 0b0010_0000]
    );
    assert_eq!(plot_spans::<u8, 1>([(1, 3, 0)]), [0b0111_0000]);
}

#[test]
fn mismatch_marks_missing_and_extra() {
    let err = std::panic::catch_unwind(|| {
        #[rustfmt::skip]
        assert_bitmap_eq!(
            [
                0b0100_0000u8,
                0b0010_0000,
            ],
            [
                0b1000_0000,
                0b0010_0000,
            ],
        );
    })
    .expect_err("grids differ");
    let msg = panic_message(&err);
    assert!(
        msg.contains("# match") && msg.contains("- missing") && msg.contains("+ extra"),
        "missing legend: {msg}"
    );
    assert!(
        msg.contains("0 | -+......"),
        "row 0 should be `-+......` (missing then extra): {msg}"
    );
    assert!(
        msg.contains("1 | ..#....."),
        "row 1 should stay a match: {msg}"
    );
    assert!(
        msg.contains("missing: (0, 0)"),
        "missing list should name (0, 0): {msg}"
    );
    assert!(
        msg.contains("extra: (1, 0)"),
        "extra list should name (1, 0): {msg}"
    );
}

#[test]
fn bitmap_diff_is_none_when_equal() {
    assert!(bitmap_diff([0b1010_0000u8], [0b1010_0000u8]).is_none());
}

#[test]
fn equal_hexmaps_pass() {
    #[rustfmt::skip]
    assert_hexmap_eq!([0x1F00u16], [0x1F00]);
    assert!(hexmap_eq([0x1F00u16], [0x1F00u16]));
}

#[test]
fn plot_hex_packs_high_nibble_left() {
    assert_eq!(plot_hex::<u16, 1>([(0, 0, 0x1), (2, 0, 0xa)]), [0x10A0]);
}

#[test]
fn hexmap_mismatch_marks_missing_extra_changed() {
    let diff = hexmap_diff([0x0AC0u16], [0xBF00u16]).expect("grids differ");
    let msg = diff.to_string();
    assert!(
        msg.contains("0 | -*+."),
        "row should be missing, changed, extra, then empty: {msg}"
    );
    assert_eq!(diff.missing, [(0, 0)]);
    assert_eq!(diff.extra, [(2, 0)]);
    assert_eq!(diff.changed, [(1, 0, 0xA, 0xF)]);
}

fn panic_message(err: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        (*s).to_string()
    } else {
        panic!("unexpected panic payload");
    }
}
