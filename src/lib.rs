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

// c_mneumonic_words: 24,18 or 12
// Return codes:  0 : Success
//               -1 : Could not set word count do desired number
//               -2 : Invalid input: word count must be 12,18 or 24
//               -3 : Internal error: Did not generate mneumonic with desired word count
//               -4 : Internal error: The seed is incorrect length
//               -5 : pca_phrase_size too small to hold the mneumonic words
//               -6 : runtime exception occurred
#[no_mangle]
pub unsafe extern "C" 
fn bip39_generate_new_seed(c_mnemonic_words: u8, pca_phrase: *mut c_void, pc_phrase_size: *const u8, pca_seed: *mut c_void ) -> i8 {
    if (c_mnemonic_words!=24) && (c_mnemonic_words!=18) && (c_mnemonic_words!=12) {
      return -2;
    }
 
    let result_a = MnemonicType::for_word_count( c_mnemonic_words.into() );
    if result_a.is_err() {
      return -6;
    }
    let mnemonic_type = result_a.unwrap();
    if mnemonic_type.word_count() != c_mnemonic_words.into() {
      return -1;
    }
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);

    let actual_word_count = mnemonic.phrase().split(" ").count();
    if actual_word_count != c_mnemonic_words.into() {
      return -3;
    }

    let seed = Seed::new(&mnemonic, "");
    let seed_bytes: &[u8] = seed.as_bytes();
    if seed_bytes.len() != 64 {
      return -4;
    }
    
    //Copy seed into return parameter
    ptr::copy_nonoverlapping(seed_bytes.as_ptr(), pca_seed as *mut u8, 64);


    //Note: CString is already null terminated, while rust str is not.
    //      Take null terminating byte into consideration in the array size comparison
    let s_mnemonic_len = mnemonic.phrase().len();
    let s_mnemonic     = CString::new( mnemonic.phrase() ).unwrap();

    let i_phrase_usize: usize = *pc_phrase_size as usize;
    if i_phrase_usize < (s_mnemonic_len+1) {
      return -5;
    }

    //Copy mnemonic into parameter, including null terminating character
    ptr::copy_nonoverlapping(s_mnemonic.as_ptr(), pca_phrase as *mut i8, s_mnemonic_len+1);
    
    return 0;
}

// pca_phrase: mnemonic string consisting of 12, 18 or 24 words
// c_phrase_length: string length of pca_phrase
// Return codes:  0 : Success
//               -1 : The supplied string length doesn't match the calculated length of pca_phrase
//               -2 : Invalid input: word count must be 12,18 or 24
//               -3 : Internal error: Incorrect phrase length
//               -4 : Internal error: Incorrect seed length
//               -5 : Internal error: Could not regenerate the seed from the input phrase
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
    if phrase_word_count!=24 && phrase_word_count!=18 && phrase_word_count!=12
    {
        return -2;
    }

    //If not explicitly handling the Err() a program abort (panic) will occur
    //instead of the library returning with the error code
    let result = Mnemonic::from_phrase(trimmed, Language::English);
    match result {
      Ok(mnemonic) => {

        let actual_word_count = mnemonic.phrase().split(" ").count();
        if actual_word_count!=phrase_word_count {
          return -3;
        }

        let seed = Seed::new(&mnemonic, "");
        let seed_bytes: &[u8] = seed.as_bytes();
        if seed_bytes.len() != 64 {
          return -4;
        }    
        //Copy seed into return parameter
        ptr::copy_nonoverlapping(seed_bytes.as_ptr(), pca_seed as *mut u8, 64);

        return 0;
      },      
      Err(_) => {return -5;}
  };      
}
