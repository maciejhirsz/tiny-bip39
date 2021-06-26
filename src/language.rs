use std::fmt;
use std::cmp::Ordering;
use crate::error::ErrorKind;
use crate::util::{Bits, Bits11};
use rustc_hash::FxHashMap;
use zeroize::Zeroize;

pub struct WordMap<'a> {
    inner: FxHashMap<&'a str, Bits11>,
}

pub struct WordList<'a> {
    inner: Vec<&'a str>,
}

impl WordMap<'_> {
    pub fn get_bits(&self, word: &str) -> Result<Bits11, ErrorKind> {
        match self.inner.get(word) {
            Some(n) => Ok(*n),
            None => Err(ErrorKind::InvalidWord)?,
        }
    }
}

impl WordList<'_> {
    pub fn get_word(&self, bits: Bits11) -> &str {
        self.inner[bits.bits() as usize]
    }

    pub fn get_words_by_prefix(&self, prefix: &str) -> &[&str] {
        let start = self.inner
            .binary_search(&prefix)
            .unwrap_or_else(|idx| idx);
        let count = self.inner[start..].iter()
            .take_while(|word| word.starts_with(prefix))
            .count();

        &self.inner[start..start + count]
    }
}

impl WordList<'static> {
    fn gen_wordmap(&self) -> WordMap<'static> {
        let inner = self
            .inner
            .iter()
            .enumerate()
            .map(|(i, item)| (*item, Bits11::from(i as u16)))
            .collect();

        WordMap { inner }
    }
}

mod lazy {
    use super::{WordList, WordMap};
    use once_cell::sync::Lazy;

    /// lazy generation of the word list
    fn gen_wordlist(lang_words: &'static str) -> WordList<'static> {
        let inner: Vec<_> = lang_words.split_whitespace().collect();

        debug_assert!(inner.len() == 2048, "Invalid wordlist length");

        WordList { inner }
    }

    pub static WORDLIST_ENGLISH: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/english.txt")));
    #[cfg(feature = "chinese-simplified")]
    pub static WORDLIST_CHINESE_SIMPLIFIED: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/chinese_simplified.txt")));
    #[cfg(feature = "chinese-traditional")]
    pub static WORDLIST_CHINESE_TRADITIONAL: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/chinese_traditional.txt")));
    #[cfg(feature = "french")]
    pub static WORDLIST_FRENCH: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/french.txt")));
    #[cfg(feature = "italian")]
    pub static WORDLIST_ITALIAN: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/italian.txt")));
    #[cfg(feature = "japanese")]
    pub static WORDLIST_JAPANESE: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/japanese.txt")));
    #[cfg(feature = "korean")]
    pub static WORDLIST_KOREAN: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/korean.txt")));
    #[cfg(feature = "spanish")]
    pub static WORDLIST_SPANISH: Lazy<WordList> =
        Lazy::new(|| gen_wordlist(include_str!("langs/spanish.txt")));

    pub static WORDMAP_ENGLISH: Lazy<WordMap> = Lazy::new(|| WORDLIST_ENGLISH.gen_wordmap());
    #[cfg(feature = "chinese-simplified")]
    pub static WORDMAP_CHINESE_SIMPLIFIED: Lazy<WordMap> =
        Lazy::new(|| WORDLIST_CHINESE_SIMPLIFIED.gen_wordmap());
    #[cfg(feature = "chinese-traditional")]
    pub static WORDMAP_CHINESE_TRADITIONAL: Lazy<WordMap> =
        Lazy::new(|| WORDLIST_CHINESE_TRADITIONAL.gen_wordmap());
    #[cfg(feature = "french")]
    pub static WORDMAP_FRENCH: Lazy<WordMap> = Lazy::new(|| WORDLIST_FRENCH.gen_wordmap());
    #[cfg(feature = "italian")]
    pub static WORDMAP_ITALIAN: Lazy<WordMap> = Lazy::new(|| WORDLIST_ITALIAN.gen_wordmap());
    #[cfg(feature = "japanese")]
    pub static WORDMAP_JAPANESE: Lazy<WordMap> = Lazy::new(|| WORDLIST_JAPANESE.gen_wordmap());
    #[cfg(feature = "korean")]
    pub static WORDMAP_KOREAN: Lazy<WordMap> = Lazy::new(|| WORDLIST_KOREAN.gen_wordmap());
    #[cfg(feature = "spanish")]
    pub static WORDMAP_SPANISH: Lazy<WordMap> = Lazy::new(|| WORDLIST_SPANISH.gen_wordmap());
}

/// The language determines which words will be used in a mnemonic phrase, but also indirectly
/// determines the binary value of each word when a [`Mnemonic`][Mnemonic] is turned into a [`Seed`][Seed].
///
/// These are not of much use right now, and may even be removed from the crate, as there is no
/// official language specified by the standard except English.
///
/// [Mnemonic]: ./mnemonic/struct.Mnemonic.html
/// [Seed]: ./seed/struct.Seed.html
#[derive(Debug, Clone, Copy, PartialEq, Zeroize)]
#[zeroize(drop)]
pub enum Language {
    #[cfg(feature = "english")]
    English,
    #[cfg(feature = "chinese-simplified")]
    ChineseSimplified,
    #[cfg(feature = "chinese-traditional")]
    ChineseTraditional,
    #[cfg(feature = "french")]
    French,
    #[cfg(feature = "italian")]
    Italian,
    #[cfg(feature = "japanese")]
    Japanese,
    #[cfg(feature = "korean")]
    Korean,
    #[cfg(feature = "spanish")]
    Spanish,
}

pub trait LangTrait: Copy + fmt::Debug {
    fn wordlist<'a>(&'a self) -> &'a WordList<'a>;

    fn wordmap<'a>(&'a self) -> &'a WordMap<'a>;
}

impl Language {
    /// Construct a word list from its language code. Returns None
    /// if the language code is not valid or not supported.
    pub fn from_language_code(language_code: &str) -> Option<Self> {
        match &language_code.to_ascii_lowercase()[..] {
            "en" => Some(Language::English),
            #[cfg(feature = "chinese-simplified")]
            "zh-hans" => Some(Language::ChineseSimplified),
            #[cfg(feature = "chinese-traditional")]
            "zh-hant" => Some(Language::ChineseTraditional),
            #[cfg(feature = "french")]
            "fr" => Some(Language::French),
            #[cfg(feature = "italian")]
            "it" => Some(Language::Italian),
            #[cfg(feature = "japanese")]
            "ja" => Some(Language::Japanese),
            #[cfg(feature = "korean")]
            "ko" => Some(Language::Korean),
            #[cfg(feature = "spanish")]
            "es" => Some(Language::Spanish),
            _ => None,
        }
    }
}

impl LangTrait for Language {
    /// Get the word list for this language
    fn wordlist<'a>(&'a self) -> &'a WordList<'a> {
        match *self {
            Language::English => &lazy::WORDLIST_ENGLISH,
            #[cfg(feature = "chinese-simplified")]
            Language::ChineseSimplified => &lazy::WORDLIST_CHINESE_SIMPLIFIED,
            #[cfg(feature = "chinese-traditional")]
            Language::ChineseTraditional => &lazy::WORDLIST_CHINESE_TRADITIONAL,
            #[cfg(feature = "french")]
            Language::French => &lazy::WORDLIST_FRENCH,
            #[cfg(feature = "italian")]
            Language::Italian => &lazy::WORDLIST_ITALIAN,
            #[cfg(feature = "japanese")]
            Language::Japanese => &lazy::WORDLIST_JAPANESE,
            #[cfg(feature = "korean")]
            Language::Korean => &lazy::WORDLIST_KOREAN,
            #[cfg(feature = "spanish")]
            Language::Spanish => &lazy::WORDLIST_SPANISH,
        }
    }

    /// Get a [`WordMap`][WordMap] that allows word -> index lookups in the word list
    ///
    /// The index of an individual word in the word list is used as the binary value of that word
    /// when the phrase is turned into a [`Seed`][Seed].
    fn wordmap<'a>(&'a self) -> &'a WordMap<'a> {
        match *self {
            Language::English => &lazy::WORDMAP_ENGLISH,
            #[cfg(feature = "chinese-simplified")]
            Language::ChineseSimplified => &lazy::WORDMAP_CHINESE_SIMPLIFIED,
            #[cfg(feature = "chinese-traditional")]
            Language::ChineseTraditional => &lazy::WORDMAP_CHINESE_TRADITIONAL,
            #[cfg(feature = "french")]
            Language::French => &lazy::WORDMAP_FRENCH,
            #[cfg(feature = "italian")]
            Language::Italian => &lazy::WORDMAP_ITALIAN,
            #[cfg(feature = "japanese")]
            Language::Japanese => &lazy::WORDMAP_JAPANESE,
            #[cfg(feature = "korean")]
            Language::Korean => &lazy::WORDMAP_KOREAN,
            #[cfg(feature = "spanish")]
            Language::Spanish => &lazy::WORDMAP_SPANISH,
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}

/// Helper that allows you to add a new dictionary for a custom language,
/// as long as the word list contains exactly 2048 sorted words.
///
/// # Example
///
/// ```
/// use bip39::{Mnemonic, MnemonicType, CustomLanguage};
///
/// // Supply your own list
/// static BIP39_ENGLISH: &str = include_str!("langs/english.txt");
///
/// let language = CustomLanguage::new(BIP39_ENGLISH).unwrap();
/// let phrase = "crop cash unable insane eight faith inflict route frame loud box vibrant";
/// let mnemonic = Mnemonic::from_phrase(phrase, &language).unwrap();
///
/// assert_eq!("33E46BB13A746EA41CDDE45C90846A79", format!("{:X}", mnemonic));
/// ```
pub struct CustomLanguage {
    _source: Box<str>,
    // We use 'static here, but really we borrow from `source`
    map: WordMap<'static>,
    // We use 'static here, but really we borrow from `source`
    list: WordList<'static>,
}

impl fmt::Debug for CustomLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.list.inner.fmt(f)
    }
}

impl CustomLanguage {
    /// Create a new `Custom` language from a single list of pre-sorted words.
    /// Words must be separate by at least one Unicode whitespace character,
    /// those include space (U+0020) and line feed (U+000A).
    ///
    /// This will return an error if the words are not sorted alphabetically,
    /// or if the total count of words isn't 2048.
    pub fn new(source: impl Into<Box<str>>) -> Result<Self, ErrorKind> {
        let source = source.into();

        let mut last_word = "";

        let inner = source
            .split_whitespace()
            .map(|word| {
                let word = unsafe {
                    // Re-borrow to get a 'static lifetime. This means that we must
                    // always narrow down the lifetime back to the lifetime of this
                    // CustomLanguage (which contains the `source`), so that the
                    // refs are guaranteed not to outlive the `source`!!
                    &*(word as *const str)
                };

                match word.cmp(last_word) {
                    Ordering::Greater => {
                        last_word = word;
                        Ok(word)
                    },
                    _ => Err(ErrorKind::InvalidOrder)
                }
            }).collect::<Result<Vec<&'static str>, _>>()?;

        if inner.len() != 2048 {
            return Err(ErrorKind::InvalidWordCount(inner.len()));
        }

        let list = WordList { inner };
        let map = list.gen_wordmap();

        Ok(Self {
            _source: source,
            map,
            list,
        })
    }
}

impl LangTrait for &CustomLanguage {
    fn wordlist<'a>(&'a self) -> &'a WordList<'a> {
        &self.list
    }

    fn wordmap<'a>(&'a self) -> &'a WordMap<'a> {
        &self.map
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "english")]
    fn words_by_prefix() {
        let wl = &lazy::WORDLIST_ENGLISH;
        let res = wl.get_words_by_prefix("woo");
        assert_eq!(res, ["wood","wool"]);
    }

    #[test]
    #[cfg(feature = "english")]
    fn all_words_by_prefix() {
        let wl = &lazy::WORDLIST_ENGLISH;
        let res = wl.get_words_by_prefix("");
        assert_eq!(res.len(), 2048);
    }

    #[test]
    #[cfg(feature = "english")]
    fn words_by_invalid_prefix() {
        let wl = &lazy::WORDLIST_ENGLISH;
        let res = wl.get_words_by_prefix("woof");
        assert!(res.is_empty());
    }

    fn is_wordlist_nfkd(wl: &WordList) -> bool {
        for idx in 0..2047 {
            let word = wl.get_word(idx.into());
            if !unicode_normalization::is_nfkd(word) {
                return false;
            }
        }
        return true;
    }

    #[test]
    #[cfg(feature = "chinese-simplified")]
    fn chinese_simplified_wordlist_is_nfkd() {
        assert!(is_wordlist_nfkd(&lazy::WORDLIST_CHINESE_SIMPLIFIED));
    }

    #[test]
    #[cfg(feature = "chinese-traditional")]
    fn chinese_traditional_wordlist_is_nfkd() {
        assert!(is_wordlist_nfkd(&lazy::WORDLIST_CHINESE_TRADITIONAL));
    }

    #[test]
    #[cfg(feature = "french")]
    fn french_wordlist_is_nfkd() {
        assert!(is_wordlist_nfkd(&lazy::WORDLIST_FRENCH));
    }

    #[test]
    #[cfg(feature = "italian")]
    fn italian_wordlist_is_nfkd() {
        assert!(is_wordlist_nfkd(&lazy::WORDLIST_ITALIAN));
    }

    #[test]
    #[cfg(feature = "japanese")]
    fn japanese_wordlist_is_nfkd() {
        assert!(is_wordlist_nfkd(&lazy::WORDLIST_JAPANESE));
    }

    #[test]
    #[cfg(feature = "korean")]
    fn korean_wordlist_is_nfkd() {
        assert!(is_wordlist_nfkd(&lazy::WORDLIST_KOREAN));
    }

    #[test]
    #[cfg(feature = "spanish")]
    fn spanish_wordlist_is_nfkd() {
        assert!(is_wordlist_nfkd(&lazy::WORDLIST_SPANISH));
    }

    #[test]
    fn from_language_code_en() {
        assert_eq!(
            Language::from_language_code("En").expect("en is a valid language"),
            Language::English
        );
    }

    #[test]
    #[cfg(feature = "chinese-simplified")]
    fn from_language_code_cn_hans() {
        assert_eq!(
            Language::from_language_code("Zh-Hans").expect("zh-hans is a valid language"),
            Language::ChineseSimplified
        );
    }

    #[test]
    #[cfg(feature = "chinese-traditional")]
    fn from_language_code_cn_hant() {
        assert_eq!(
            Language::from_language_code("zh-hanT").expect("zh-hant is a valid language"),
            Language::ChineseTraditional
        );
    }

    #[test]
    #[cfg(feature = "french")]
    fn from_language_code_fr() {
        assert_eq!(
            Language::from_language_code("fr").expect("fr is a valid language"),
            Language::French
        );
    }

    #[test]
    #[cfg(feature = "italian")]
    fn from_language_code_it() {
        assert_eq!(
            Language::from_language_code("It").expect("it is a valid language"),
            Language::Italian
        );
    }

    #[test]
    #[cfg(feature = "japanese")]
    fn from_language_code_ja() {
        assert_eq!(
            Language::from_language_code("Ja").expect("ja is a valid language"),
            Language::Japanese
        );
    }

    #[test]
    #[cfg(feature = "korean")]
    fn from_language_code_ko() {
        assert_eq!(
            Language::from_language_code("kO").expect("ko is a valid language"),
            Language::Korean
        );
    }

    #[test]
    #[cfg(feature = "spanish")]
    fn from_language_code_es() {
        assert_eq!(
            Language::from_language_code("ES").expect("es is a valid language"),
            Language::Spanish
        );
    }

    #[test]
    fn from_invalid_language_code() {
        assert_eq!(Language::from_language_code("not a real language"), None);
    }

    #[test]
    fn custom_language_invalid_order() {
        assert_eq!(CustomLanguage::new("alpha\ngamma\nbeta").unwrap_err(), ErrorKind::InvalidOrder);
    }

    #[test]
    fn custom_language_invalid_count() {
        assert_eq!(CustomLanguage::new("alpha\nbeta\ngamma").unwrap_err(), ErrorKind::InvalidWordCount(3));
    }

    #[test]
    #[cfg(feature = "english")]
    fn custom_language_english() {
        let lang = CustomLanguage::new(include_str!("langs/english.txt")).unwrap();

        assert_eq!(&lang.list.inner, &lazy::WORDLIST_ENGLISH.inner);
    }
}
