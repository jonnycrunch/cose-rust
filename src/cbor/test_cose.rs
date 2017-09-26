use cbor::decoder::*;

#[test]
fn test_single_signature() {
    let bytes: Vec<u8> = vec![
        0xD8, 0x62,
            0x84,
                0x40,                                // bytes(0)
                                                     // ""
                0xA0,                                // map(0)
                0x54,                                // bytes(20)
                    0x54, 0x68, 0x69, 0x73, 0x20,    // "This is the content."
                    0x69, 0x73, 0x20, 0x74, 0x68,
                    0x65, 0x20, 0x63, 0x6F, 0x6E,
                    0x74, 0x65, 0x6E, 0x74, 0x2E,
                0x81,                                // array(1)
                    0x83,                            // array(3)
                        0x43,                        // bytes(3)
                            0xA1, 0x01, 0x26,        // "\xA1\x01&"
                        0xA1,                        // map(1)
                            0x04,                    // unsigned(4)
                            0x42,                    // bytes(2)
                                0x31, 0x31,          // "11"
                        0x58, 0x40,                  // bytes(64)
                            0xe2, 0xae, 0xaf, 0xd4,  // signature bytes
                            0x0d, 0x69, 0xd1, 0x9d,
                            0xfe, 0x6e, 0x52, 0x07,
                            0x7c, 0x5d, 0x7f, 0xf4,
                            0xe4, 0x08, 0x28, 0x2c,
                            0xbe, 0xfb, 0x5d, 0x06,
                            0xcb, 0xf4, 0x14, 0xaf,
                            0x2e, 0x19, 0xd9, 0x82,
                            0xac, 0x45, 0xac, 0x98,
                            0xb8, 0x54, 0x4c, 0x90,
                            0x8b, 0x45, 0x07, 0xde,
                            0x1e, 0x90, 0xb7, 0x17,
                            0xc3, 0xd3, 0x48, 0x16,
                            0xfe, 0x92, 0x6a, 0x2b,
                            0x98, 0xf5, 0x3a, 0xfd,
                            0x2f, 0xa0, 0xf3, 0x0a
    ];
    let cose_signature = decode_signature(bytes).unwrap();
}
