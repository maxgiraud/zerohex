use zerohex::Zerohex;

#[derive(Zerohex, PartialEq)]
struct Address([u8;20]);

#[test]
fn test_display() {
    let address = Address([1u8;20]);
    
    assert_eq!(
        format!("{}", address),
        "0x0101010101010101010101010101010101010101"
    );
}

#[test]
fn test_fromstr_no_prefix() -> Result<(), FromHexError> {
    assert_eq!(
        Address::from_str("000102030405060708090a0b0c0d0e0f10111213")?,
        Address([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
            0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13
        ])
    );
    Ok(())
}

#[test]
fn test_fromstr_prefix() -> Result<(), FromHexError> {
    assert_eq!(
        Address::from_str("0x000102030405060708090a0b0c0d0e0f10111213")?,
        Address([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
            0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13
        ])
    );
    Ok(())
}
