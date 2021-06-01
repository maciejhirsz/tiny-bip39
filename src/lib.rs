//!
//! This is a Rust implementation of the [bip39][bip39-standard] standard for Bitcoin HD wallet
//! mnemonic phrases.
//!
//!
//! [bip39-standard]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
//!
//! ## Quickstart
//!
//! ```rust
//! use bip39::{Mnemonic, MnemonicType, Language, Seed};
//!
//! /// create a new randomly generated mnemonic phrase
//! let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
//!
//! /// get the phrase
//! let phrase: &str = mnemonic.phrase();
//! println!("phrase: {}", phrase);
//!
//! /// get the HD wallet seed
//! let seed = Seed::new(&mnemonic, "");
//!
//! // get the HD wallet seed as raw bytes
//! let seed_bytes: &[u8] = seed.as_bytes();
//!
//! // print the HD wallet seed as a hex string
//! println!("{:X}", seed);
//! ```
//!
mod error;
mod language;
mod mnemonic;
mod mnemonic_type;
mod seed;
mod util;

mod crypto;

pub use error::ErrorKind;
pub use language::Language;
pub use mnemonic::Mnemonic;
pub use mnemonic_type::MnemonicType;
pub use seed::Seed;


use std::ptr;
use libc::c_void;
use std::ffi::CString;
use std::ffi::CStr;


#[no_mangle]
pub unsafe extern "C" 
fn bip39_generate_new_seed(pca_phrase: *mut c_void, c_phrase_size: *const u8, pca_seed: *mut c_void ) -> i8 {
    let mnemonic_type = MnemonicType::for_word_count(24).unwrap();
    if mnemonic_type.word_count() != 24 {
      return -1;
    }

    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    let actual_word_count = mnemonic.phrase().split(" ").count();
    if actual_word_count != 24 {
      return -1;
    }

    let seed = Seed::new(&mnemonic, "");
    let seed_bytes: &[u8] = seed.as_bytes();
    if seed_bytes.len() != 64 {
      return -1;
    }
    
    //Copy seed into return parameter
    ptr::copy_nonoverlapping(seed_bytes.as_ptr(), pca_seed as *mut u8, 64);


    //Note: CString is already null terminated, while rust str is not.
    //      Take null terminating byte into consideration in the array size comparison
    let s_mnemonic_len = mnemonic.phrase().len();
    let s_mnemonic     = CString::new( mnemonic.phrase() ).unwrap();

    let i_phrase_usize: usize = *c_phrase_size as usize;
    if i_phrase_usize < (s_mnemonic_len+1) {
      return -2;
    }

    //Copy mnemonic into parameter, including null terminating character
    ptr::copy_nonoverlapping(s_mnemonic.as_ptr(), pca_phrase as *mut i8, s_mnemonic_len+1);
    
    return 0;
}

#[no_mangle]
pub unsafe extern "C" 
fn bip39_regenerate_seed_from_mnemonic(pca_phrase: *mut c_void, c_phrase_length: *const u8, pca_seed: *mut c_void) -> i8 {
    let i_phrase_usize: usize = *c_phrase_length as usize;

    let cs_mnemonic = CStr::from_ptr( pca_phrase as *mut i8 );
    if cs_mnemonic.to_bytes().len() != i_phrase_usize {
      return -1;
    }
    
    let s_menmonic = cs_mnemonic.to_str().unwrap();
    let trimmed = s_menmonic.trim();
    let phrase_word_count =trimmed.split(" ").count();
    if phrase_word_count!=24 {
        return -1;
    }
      
    let mnemonic = Mnemonic::from_phrase(trimmed, Language::English).expect("Can create a Mnemonic");
    let actual_word_count = mnemonic.phrase().split(" ").count();
    if actual_word_count!=24 {
        return -2;
    }

    let seed = Seed::new(&mnemonic, "");
    let seed_bytes: &[u8] = seed.as_bytes();
    if seed_bytes.len() != 64 {
      return -2;
    }    
    //Copy seed into return parameter
    ptr::copy_nonoverlapping(seed_bytes.as_ptr(), pca_seed as *mut u8, 64);

    return 0;
}
