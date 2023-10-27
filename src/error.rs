use crate::mnemonic_type::MnemonicType;
use core::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    InvalidChecksum,
    InvalidWord,
    InvalidKeysize(usize),
    InvalidWordLength(usize),
    InvalidEntropyLength(usize, MnemonicType),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChecksum => write!(f, "invalid checksum"),
            Self::InvalidWord => write!(f, "invalid word in phrase"),
            Self::InvalidKeysize(u) => write!(f, "invalid keysize: {0}", u),
            Self::InvalidWordLength(u) => write!(f, "invalid number of words in phrase: {0}", u),
            Self::InvalidEntropyLength(u, m) => write!(
                f,
                "invalid entropy length {0}bits for mnemonic type {1:?}",
                u, m
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[cfg_attr(all(target_arch = "wasm32"), wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn prints_correctly() {
        assert_eq!(
            format!("{}", ErrorKind::InvalidChecksum),
            "invalid checksum",
        );
        assert_eq!(
            format!("{}", ErrorKind::InvalidKeysize(42)),
            "invalid keysize: 42",
        );
        assert_eq!(
            format!(
                "{}",
                ErrorKind::InvalidEntropyLength(42, MnemonicType::Words12)
            ),
            "invalid entropy length 42bits for mnemonic type Words12",
        );
    }
}
