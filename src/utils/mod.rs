/// Functions for convert vectors between u32 and u8
pub mod vectors;
/// Functions for serialize string and integers
pub mod serialize;
/// Functions for encode and decode with base58
pub mod base58;
/// Functions for get vetor of random numbers
pub mod random;
/// Functions low leval of cryptography
pub mod crypto;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}