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

// curve: NIST P-256
// public key: U = xG
// Ux = 60FED4BA255A9D31C961EB74C6356D68C049B8923B61FA6CE669622E60F29FB6
// Uy = 7903FE1008B8BC99A41AE9E95628BC64F2F1B20C2D7E9F5177A3C294D4462299
// SEQUENCE
//   SEQUENCE
//     OID: 1.2.840.10045.2.1 (ecPublicKey)
//     OID: 1.2.840.10045.3.1.7 (NIST P-256)
//  BITSTRING (uncompressed EC point)
pub static NIST_P256_TEST_SPKI: &'static [u8] =
    &[0x30, 0x59,
            0x30, 0x13,
                  0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01,
                  0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07,
            0x03, 0x42,
                  0x00, // 0 unused bits
                  0x04, // uncompressed form
                  0x60, 0xfe, 0xd4, 0xba, 0x25, 0x5a, 0x9d, 0x31, 0xc9, 0x61, 0xeb, 0x74,
                        0xc6, 0x35, 0x6d, 0x68, 0xc0, 0x49, 0xb8, 0x92, 0x3b, 0x61, 0xfa,
                        0x6c, 0xe6, 0x69, 0x62, 0x2e, 0x60, 0xf2, 0x9f, 0xb6,
                  0x79, 0x03, 0xfe, 0x10, 0x08, 0xb8, 0xbc, 0x99, 0xa4, 0x1a, 0xe9, 0xe9,
                        0x56, 0x28, 0xbc, 0x64, 0xf2, 0xf1, 0xb2, 0x0c, 0x2d, 0x7e, 0x9f,
                        0x51, 0x77, 0xa3, 0xc2, 0x94, 0xd4, 0x46, 0x22, 0x99];
