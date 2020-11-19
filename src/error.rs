use crate::mnemonic_type::MnemonicType;
use thiserror::Error;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("invalid checksum")]
    InvalidChecksum,
    #[error("invalid word in phrase")]
    InvalidWord,
    #[error("word list is unordered")]
    InvalidOrder,
    #[error("found {0} words in source, expected 2048")]
    InvalidWordCount(usize),
    #[error("invalid keysize: {0}")]
    InvalidKeysize(usize),
    #[error("invalid number of words in phrase: {0}")]
    InvalidWordLength(usize),
    #[error("invalid entropy length {0}bits for mnemonic type {1:?}")]
    InvalidEntropyLength(usize, MnemonicType),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
            format!("{}", ErrorKind::InvalidEntropyLength(42, MnemonicType::Words12)),
            "invalid entropy length 42bits for mnemonic type Words12",
        );
    }
}
