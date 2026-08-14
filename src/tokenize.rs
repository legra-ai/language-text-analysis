//! Tokenization on Unicode word boundaries (UAX #29).

use unicode_segmentation::UnicodeSegmentation;

/// Split already-[normalized](crate) text into word tokens on Unicode
/// word boundaries.
///
/// Punctuation and whitespace segments are dropped. UAX #29 breaks
/// between CJK ideographs, so each Han character becomes its own token
/// (character-level CJK tokenization); mixed-script text segments by
/// each script's own rules.
pub(crate) fn tokenize(text: &str) -> impl Iterator<Item = &str> {
    text.unicode_words()
}
