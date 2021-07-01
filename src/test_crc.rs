use crate::crc::*;

#[test]
fn test_fill() {
    helper_fill([0x01, 0x06, 0x00, 0x30, 0x00, 0xb4, 0x00, 0x00], [0x01, 0x06, 0x00, 0x30, 0x00, 0xb4, 0x89, 0xb2]);

    helper_fill([0x01, 0x06, 0x00, 0x30, 0x00, 0xb4, 0x12, 0x34], [0x01, 0x06, 0x00, 0x30, 0x00, 0xb4, 0x89, 0xb2]);

    helper_fill([0x01, 0x06, 0x00, 0x30, 0x00, 0xf0, 0x00, 0x00], [0x01, 0x06, 0x00, 0x30, 0x00, 0xf0, 0x89, 0x81]);

    helper_fill([0x01, 0x06, 0x00, 0x30, 0x01, 0x18, 0x00, 0x00], [0x01, 0x06, 0x00, 0x30, 0x01, 0x18, 0x88, 0x5f]);
}

fn helper_fill(input: [u8; MESSAGE_LENGTH], output: [u8; MESSAGE_LENGTH]) {
    let mut message = input;
    fill(&mut message);
    assert_eq!(output, message);
}
