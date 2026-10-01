use bitlit::{
    assert_bitmap_eq, assert_hexmap_eq, bitmap_diff, bitmap_eq, hexmap_diff, hexmap_eq, plot_bits,
    plot_hex, plot_spans,
};

#[test]
fn equal_grids_pass() {
    assert_bitmap_eq!([0b1010u8, 0b0101], [0b1010u8, 0b0101], 4);
}

#[test]
fn plot_bits_packs_msb_left() {
    assert_eq!(plot_bits::<2>([(0, 0), (2, 1)], 3), [0b100, 0b001]);
    assert_eq!(plot_spans::<1>([(1, 3, 0)], 5), [0b01110]);
}

#[test]
fn bitmap_eq_masks_unused_high_bits() {
    assert!(bitmap_eq([0b11010u8], [0b01010u8], 4));
    assert!(!bitmap_eq([0b11010u8], [0b01010u8], 5));
}

#[test]
fn bitmap_eq_rejects_row_count_mismatch() {
    assert!(!bitmap_eq([0b1u8, 0b0], [0b1u8], 1));
}

#[test]
fn mismatch_marks_missing_and_extra() {
    let err = std::panic::catch_unwind(|| {
        assert_bitmap_eq!([0b0100u8, 0b0010], [0b1000u8, 0b0010], 4);
    })
    .expect_err("grids differ");
    let msg = panic_message(&err);
    assert!(
        msg.contains("# match") && msg.contains("- missing") && msg.contains("+ extra"),
        "missing legend: {msg}"
    );
    assert!(
        msg.contains("0 | -+.."),
        "row 0 should be `-+..` (missing then extra): {msg}"
    );
    assert!(msg.contains("1 | ..#."), "row 1 should stay a match: {msg}");
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
    assert!(bitmap_diff([0b1010u8], [0b1010u8], 4).is_none());
}

#[test]
fn equal_hexmaps_pass() {
    assert_hexmap_eq!([0x1F0u16], [0x1F0u16], 3);
    assert!(hexmap_eq([0x1F0u16], [0x1F0u16], 3));
}

#[test]
fn plot_hex_packs_high_nibble_left() {
    assert_eq!(plot_hex::<1>([(0, 0, 0x1), (2, 0, 0xa)], 3), [0x10A]);
}

#[test]
fn hexmap_eq_masks_unused_high_nibbles() {
    assert!(hexmap_eq([0xF1A0u16], [0x01A0u16], 3));
    assert!(!hexmap_eq([0xF1A0u16], [0x01A0u16], 4));
}

#[test]
fn hexmap_mismatch_marks_missing_extra_changed() {
    let diff = hexmap_diff([0x0ACu16], [0xBF0u16], 3).expect("grids differ");
    let msg = diff.to_string();
    assert!(
        msg.contains("0 | -*+"),
        "row should be missing, changed, extra: {msg}"
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
