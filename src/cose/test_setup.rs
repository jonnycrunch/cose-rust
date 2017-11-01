use std::os::raw;
use std::ptr;
use std::sync::{ONCE_INIT, Once};
static START: Once = ONCE_INIT;

type SECStatus = raw::c_int;
const SEC_SUCCESS: SECStatus = 0;
// TODO: ugh this will probably have a platform-specific name...
#[link(name = "nss3")]
extern "C" {
    fn NSS_NoDB_Init(configdir: *const u8) -> SECStatus;
}

pub fn setup() {
    START.call_once(|| {
        let null_ptr: *const u8 = ptr::null();
        unsafe {
            assert_eq!(NSS_NoDB_Init(null_ptr), SEC_SUCCESS);
        }
    });
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const PKCS8_P256_EE: [u8; 139] = [
    0x30, 0x81, 0x87, 0x02, 0x01, 0x00, 0x30, 0x13, 0x06, 0x07, 0x2a,
    0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48,
    0xce, 0x3d, 0x03, 0x01, 0x07, 0x04, 0x6d, 0x30, 0x6b, 0x02, 0x01,
    0x01, 0x04, 0x20, 0x21, 0x91, 0x40, 0x3d, 0x57, 0x10, 0xbf, 0x15,
    0xa2, 0x65, 0x81, 0x8c, 0xd4, 0x2e, 0xd6, 0xfe, 0xdf, 0x09, 0xad,
    0xd9, 0x2d, 0x78, 0xb1, 0x8e, 0x7a, 0x1e, 0x9f, 0xeb, 0x95, 0x52,
    0x47, 0x02, 0xa1, 0x44, 0x03, 0x42, 0x00, 0x04, 0x4f, 0xbf, 0xbb,
    0xbb, 0x61, 0xe0, 0xf8, 0xf9, 0xb1, 0xa6, 0x0a, 0x59, 0xac, 0x87,
    0x04, 0xe2, 0xec, 0x05, 0x0b, 0x42, 0x3e, 0x3c, 0xf7, 0x2e, 0x92,
    0x3f, 0x2c, 0x4f, 0x79, 0x4b, 0x45, 0x5c, 0x2a, 0x69, 0xd2, 0x33,
    0x45, 0x6c, 0x36, 0xc4, 0x11, 0x9d, 0x07, 0x06, 0xe0, 0x0e, 0xed,
    0xc8, 0xd1, 0x93, 0x90, 0xd7, 0x99, 0x1b, 0x7b, 0x2d, 0x07, 0xa3,
    0x04, 0xea, 0xa0, 0x4a, 0xa6, 0xc0, 0x0a
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const P256_EE: [u8; 300] = [
    0x30, 0x82, 0x01, 0x28, 0x30, 0x81, 0xcf, 0xa0, 0x03, 0x02, 0x01, 0x02,
    0x02, 0x14, 0x2f, 0xc3, 0x5f, 0x05, 0x80, 0xb4, 0x49, 0x45, 0x13, 0x92,
    0xd6, 0x93, 0xb7, 0x2d, 0x71, 0x19, 0xc5, 0x8c, 0x40, 0x39, 0x30, 0x0a,
    0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x30, 0x13,
    0x31, 0x11, 0x30, 0x0f, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x08, 0x69,
    0x6e, 0x74, 0x2d, 0x70, 0x32, 0x35, 0x36, 0x30, 0x22, 0x18, 0x0f, 0x32,
    0x30, 0x31, 0x32, 0x30, 0x31, 0x30, 0x33, 0x30, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x5a, 0x18, 0x0f, 0x32, 0x30, 0x32, 0x31, 0x31, 0x32, 0x33, 0x31,
    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x30, 0x12, 0x31, 0x10, 0x30,
    0x0e, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x07, 0x65, 0x65, 0x2d, 0x70,
    0x32, 0x35, 0x36, 0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48,
    0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03,
    0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0x4f, 0xbf, 0xbb, 0xbb, 0x61, 0xe0,
    0xf8, 0xf9, 0xb1, 0xa6, 0x0a, 0x59, 0xac, 0x87, 0x04, 0xe2, 0xec, 0x05,
    0x0b, 0x42, 0x3e, 0x3c, 0xf7, 0x2e, 0x92, 0x3f, 0x2c, 0x4f, 0x79, 0x4b,
    0x45, 0x5c, 0x2a, 0x69, 0xd2, 0x33, 0x45, 0x6c, 0x36, 0xc4, 0x11, 0x9d,
    0x07, 0x06, 0xe0, 0x0e, 0xed, 0xc8, 0xd1, 0x93, 0x90, 0xd7, 0x99, 0x1b,
    0x7b, 0x2d, 0x07, 0xa3, 0x04, 0xea, 0xa0, 0x4a, 0xa6, 0xc0, 0x30, 0x0a,
    0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x03, 0x48,
    0x00, 0x30, 0x45, 0x02, 0x20, 0x5c, 0x75, 0x51, 0x9f, 0x13, 0x11, 0x50,
    0xcd, 0x5d, 0x8a, 0xde, 0x20, 0xa3, 0xbc, 0x06, 0x30, 0x91, 0xff, 0xb2,
    0x73, 0x75, 0x5f, 0x31, 0x64, 0xec, 0xfd, 0xcb, 0x42, 0x80, 0x0a, 0x70,
    0xe6, 0x02, 0x21, 0x00, 0xff, 0x81, 0xbe, 0xa8, 0x0d, 0x03, 0x36, 0x6b,
    0x75, 0xe2, 0x70, 0x6a, 0xac, 0x07, 0x2e, 0x4c, 0xdc, 0xf9, 0xc5, 0x89,
    0xc1, 0xcf, 0x88, 0xc2, 0xc8, 0x2a, 0x32, 0xf5, 0x42, 0x0c, 0xfa, 0x0b
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const P384_EE: [u8; 328] = [
    0x30, 0x82, 0x01, 0x44, 0x30, 0x81, 0xec, 0xa0, 0x03, 0x02, 0x01, 0x02,
    0x02, 0x14, 0x3f, 0x9d, 0xdf, 0xea, 0xe5, 0xea, 0x19, 0xab, 0xbf, 0xa0,
    0x23, 0x4e, 0x44, 0x6a, 0xc2, 0x67, 0x59, 0x7f, 0x53, 0x92, 0x30, 0x0a,
    0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x03, 0x30, 0x13,
    0x31, 0x11, 0x30, 0x0f, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x08, 0x69,
    0x6e, 0x74, 0x2d, 0x70, 0x32, 0x35, 0x36, 0x30, 0x22, 0x18, 0x0f, 0x32,
    0x30, 0x31, 0x32, 0x30, 0x31, 0x30, 0x33, 0x30, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x5a, 0x18, 0x0f, 0x32, 0x30, 0x32, 0x31, 0x31, 0x32, 0x33, 0x31,
    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x30, 0x12, 0x31, 0x10, 0x30,
    0x0e, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x07, 0x65, 0x65, 0x2d, 0x70,
    0x32, 0x35, 0x36, 0x30, 0x76, 0x30, 0x10, 0x06, 0x07, 0x2a, 0x86, 0x48,
    0xce, 0x3d, 0x02, 0x01, 0x06, 0x05, 0x2b, 0x81, 0x04, 0x00, 0x22, 0x03,
    0x62, 0x00, 0x04, 0xa1, 0x68, 0x72, 0x43, 0x36, 0x2b, 0x5c, 0x7b, 0x18,
    0x89, 0xf3, 0x79, 0x15, 0x46, 0x15, 0xa1, 0xc7, 0x3f, 0xb4, 0x8d, 0xee,
    0x86, 0x3e, 0x02, 0x29, 0x15, 0xdb, 0x60, 0x8e, 0x25, 0x2d, 0xe4, 0xb7,
    0x13, 0x2d, 0xa8, 0xce, 0x98, 0xe8, 0x31, 0x53, 0x4e, 0x6a, 0x9c, 0x0c,
    0x0b, 0x09, 0xc8, 0xd6, 0x39, 0xad, 0xe8, 0x32, 0x06, 0xe5, 0xba, 0x81,
    0x34, 0x73, 0xa1, 0x1f, 0xa3, 0x30, 0xe0, 0x5d, 0xa8, 0xc9, 0x6e, 0x43,
    0x83, 0xfe, 0x27, 0x87, 0x3d, 0xa9, 0x71, 0x03, 0xbe, 0x28, 0x88, 0xcf,
    0xf0, 0x02, 0xf0, 0x5a, 0xf7, 0x1a, 0x1f, 0xdd, 0xcc, 0x83, 0x74, 0xaa,
    0x6e, 0xa9, 0xce, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x04, 0x03, 0x03, 0x03, 0x47, 0x00, 0x30, 0x44, 0x02, 0x20, 0x5c, 0x75,
    0x51, 0x9f, 0x13, 0x11, 0x50, 0xcd, 0x5d, 0x8a, 0xde, 0x20, 0xa3, 0xbc,
    0x06, 0x30, 0x91, 0xff, 0xb2, 0x73, 0x75, 0x5f, 0x31, 0x64, 0xec, 0xfd,
    0xcb, 0x42, 0x80, 0x0a, 0x70, 0xe6, 0x02, 0x20, 0x40, 0xde, 0xda, 0x06,
    0x5a, 0xea, 0x74, 0x76, 0xd0, 0x79, 0xf9, 0xee, 0xd7, 0x3b, 0xf5, 0xd6,
    0x58, 0xed, 0x9a, 0xde, 0x49, 0x6c, 0x0f, 0xa4, 0x10, 0x1d, 0x77, 0x94,
    0x47, 0x8d, 0x04, 0xaf
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const P256_INT: [u8; 332] = [
    0x30, 0x82, 0x01, 0x48, 0x30, 0x81, 0xf0, 0xa0, 0x03, 0x02, 0x01,
    0x02, 0x02, 0x14, 0x43, 0x63, 0x59, 0xad, 0x04, 0x34, 0x56, 0x80,
    0x43, 0xec, 0x90, 0x6a, 0xd4, 0x10, 0x64, 0x7c, 0x7f, 0x38, 0x32,
    0xe2, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04,
    0x03, 0x02, 0x30, 0x14, 0x31, 0x12, 0x30, 0x10, 0x06, 0x03, 0x55,
    0x04, 0x03, 0x0c, 0x09, 0x72, 0x6f, 0x6f, 0x74, 0x2d, 0x70, 0x32,
    0x35, 0x36, 0x30, 0x22, 0x18, 0x0f, 0x32, 0x30, 0x31, 0x32, 0x30,
    0x31, 0x30, 0x33, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x18,
    0x0f, 0x32, 0x30, 0x32, 0x31, 0x31, 0x32, 0x33, 0x31, 0x30, 0x30,
    0x30, 0x30, 0x30, 0x30, 0x5a, 0x30, 0x13, 0x31, 0x11, 0x30, 0x0f,
    0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x08, 0x69, 0x6e, 0x74, 0x2d,
    0x70, 0x32, 0x35, 0x36, 0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a,
    0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48,
    0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0x4f, 0xbf,
    0xbb, 0xbb, 0x61, 0xe0, 0xf8, 0xf9, 0xb1, 0xa6, 0x0a, 0x59, 0xac,
    0x87, 0x04, 0xe2, 0xec, 0x05, 0x0b, 0x42, 0x3e, 0x3c, 0xf7, 0x2e,
    0x92, 0x3f, 0x2c, 0x4f, 0x79, 0x4b, 0x45, 0x5c, 0x2a, 0x69, 0xd2,
    0x33, 0x45, 0x6c, 0x36, 0xc4, 0x11, 0x9d, 0x07, 0x06, 0xe0, 0x0e,
    0xed, 0xc8, 0xd1, 0x93, 0x90, 0xd7, 0x99, 0x1b, 0x7b, 0x2d, 0x07,
    0xa3, 0x04, 0xea, 0xa0, 0x4a, 0xa6, 0xc0, 0xa3, 0x1d, 0x30, 0x1b,
    0x30, 0x0c, 0x06, 0x03, 0x55, 0x1d, 0x13, 0x04, 0x05, 0x30, 0x03,
    0x01, 0x01, 0xff, 0x30, 0x0b, 0x06, 0x03, 0x55, 0x1d, 0x0f, 0x04,
    0x04, 0x03, 0x02, 0x01, 0x06, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86,
    0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x03, 0x47, 0x00, 0x30, 0x44,
    0x02, 0x20, 0x63, 0x59, 0x02, 0x01, 0x89, 0xd7, 0x3e, 0x5b, 0xff,
    0xd1, 0x16, 0x4e, 0xe3, 0xe2, 0x0a, 0xe0, 0x4a, 0xd8, 0x75, 0xaf,
    0x77, 0x5c, 0x93, 0x60, 0xba, 0x10, 0x1f, 0x97, 0xdd, 0x27, 0x2d,
    0x24, 0x02, 0x20, 0x3d, 0x87, 0x0f, 0xac, 0x22, 0x4d, 0x16, 0xd9,
    0xa1, 0x95, 0xbb, 0x56, 0xe0, 0x21, 0x05, 0x93, 0xd1, 0x07, 0xb5,
    0x25, 0x3b, 0xf4, 0x57, 0x20, 0x87, 0x13, 0xa2, 0xf7, 0x78, 0x15,
    0x30, 0xa7
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const P256_ROOT: [u8; 334] = [
    0x30, 0x82, 0x01, 0x4a, 0x30, 0x81, 0xf1, 0xa0, 0x03, 0x02, 0x01, 0x02,
    0x02, 0x14, 0x5f, 0x3f, 0xae, 0x90, 0x49, 0x30, 0x2f, 0x33, 0x6e, 0x95,
    0x23, 0xa7, 0xcb, 0x23, 0xd7, 0x65, 0x4f, 0xea, 0x3c, 0xf7, 0x30, 0x0a,
    0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x30, 0x14,
    0x31, 0x12, 0x30, 0x10, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x09, 0x72,
    0x6f, 0x6f, 0x74, 0x2d, 0x70, 0x32, 0x35, 0x36, 0x30, 0x22, 0x18, 0x0f,
    0x32, 0x30, 0x31, 0x32, 0x30, 0x31, 0x30, 0x33, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x30, 0x5a, 0x18, 0x0f, 0x32, 0x30, 0x32, 0x31, 0x31, 0x32, 0x33,
    0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x30, 0x14, 0x31, 0x12,
    0x30, 0x10, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x09, 0x72, 0x6f, 0x6f,
    0x74, 0x2d, 0x70, 0x32, 0x35, 0x36, 0x30, 0x59, 0x30, 0x13, 0x06, 0x07,
    0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48,
    0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0x4f, 0xbf, 0xbb,
    0xbb, 0x61, 0xe0, 0xf8, 0xf9, 0xb1, 0xa6, 0x0a, 0x59, 0xac, 0x87, 0x04,
    0xe2, 0xec, 0x05, 0x0b, 0x42, 0x3e, 0x3c, 0xf7, 0x2e, 0x92, 0x3f, 0x2c,
    0x4f, 0x79, 0x4b, 0x45, 0x5c, 0x2a, 0x69, 0xd2, 0x33, 0x45, 0x6c, 0x36,
    0xc4, 0x11, 0x9d, 0x07, 0x06, 0xe0, 0x0e, 0xed, 0xc8, 0xd1, 0x93, 0x90,
    0xd7, 0x99, 0x1b, 0x7b, 0x2d, 0x07, 0xa3, 0x04, 0xea, 0xa0, 0x4a, 0xa6,
    0xc0, 0xa3, 0x1d, 0x30, 0x1b, 0x30, 0x0c, 0x06, 0x03, 0x55, 0x1d, 0x13,
    0x04, 0x05, 0x30, 0x03, 0x01, 0x01, 0xff, 0x30, 0x0b, 0x06, 0x03, 0x55,
    0x1d, 0x0f, 0x04, 0x04, 0x03, 0x02, 0x01, 0x06, 0x30, 0x0a, 0x06, 0x08,
    0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x03, 0x48, 0x00, 0x30,
    0x45, 0x02, 0x20, 0x5c, 0x75, 0x51, 0x9f, 0x13, 0x11, 0x50, 0xcd, 0x5d,
    0x8a, 0xde, 0x20, 0xa3, 0xbc, 0x06, 0x30, 0x91, 0xff, 0xb2, 0x73, 0x75,
    0x5f, 0x31, 0x64, 0xec, 0xfd, 0xcb, 0x42, 0x80, 0x0a, 0x70, 0xe6, 0x02,
    0x21, 0x00, 0xc2, 0xe4, 0xc1, 0xa8, 0xe2, 0x89, 0xdc, 0xa1, 0xbb, 0xe7,
    0xd5, 0x4f, 0x5c, 0x88, 0xad, 0xeb, 0xa4, 0x78, 0xa1, 0x19, 0xbe, 0x22,
    0x54, 0xc8, 0x9f, 0xef, 0xb8, 0x5d, 0xa2, 0x40, 0xd9, 0x8b
];
