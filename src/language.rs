//! Supported analysis languages and BCP-47 tag resolution.

use rust_stemmers::Algorithm;

use crate::stopwords::{english, german};

/// A language the analyzer can stem and stop-word-filter for.
///
/// Selected from a BCP-47 language tag (`de`, `en-US`) when present;
/// untagged text uses the analyzer's configured default. The set mirrors
/// the supported Snowball stemmer algorithms and is `#[non_exhaustive]`
/// so more can be added without breaking callers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Language {
    /// English.
    English,
    /// German.
    German,
    /// French.
    French,
    /// Spanish.
    Spanish,
    /// Italian.
    Italian,
    /// Dutch.
    Dutch,
    /// Portuguese.
    Portuguese,
    /// Swedish.
    Swedish,
    /// Norwegian.
    Norwegian,
    /// Danish.
    Danish,
    /// Finnish.
    Finnish,
    /// Russian.
    Russian,
}

impl Language {
    /// Resolve a BCP-47 language tag to a supported [`Language`].
    ///
    /// Only the primary subtag is significant: `en`, `en-US`, and
    /// `en_GB` all resolve to [`Language::English`]. An unsupported or
    /// malformed tag yields `None`, leaving the caller to fall back to
    /// its configured default.
    #[must_use]
    pub fn from_tag(tag: &str) -> Option<Self> {
        let primary = tag.split(['-', '_']).next().unwrap_or(tag);
        match primary.to_ascii_lowercase().as_str() {
            "en" => Some(Self::English),
            "de" => Some(Self::German),
            "fr" => Some(Self::French),
            "es" => Some(Self::Spanish),
            "it" => Some(Self::Italian),
            "nl" => Some(Self::Dutch),
            "pt" => Some(Self::Portuguese),
            "sv" => Some(Self::Swedish),
            "no" | "nb" | "nn" => Some(Self::Norwegian),
            "da" => Some(Self::Danish),
            "fi" => Some(Self::Finnish),
            "ru" => Some(Self::Russian),
            _ => None,
        }
    }

    /// The Snowball stemmer algorithm for this language.
    pub(crate) fn stemmer_algorithm(self) -> Algorithm {
        match self {
            Self::English => Algorithm::English,
            Self::German => Algorithm::German,
            Self::French => Algorithm::French,
            Self::Spanish => Algorithm::Spanish,
            Self::Italian => Algorithm::Italian,
            Self::Dutch => Algorithm::Dutch,
            Self::Portuguese => Algorithm::Portuguese,
            Self::Swedish => Algorithm::Swedish,
            Self::Norwegian => Algorithm::Norwegian,
            Self::Danish => Algorithm::Danish,
            Self::Finnish => Algorithm::Finnish,
            Self::Russian => Algorithm::Russian,
        }
    }

    /// The default stop-word list for this language, already normalized
    /// to the post-[`normalize`](crate) token form. Languages without a
    /// bundled list return an empty slice (no removal).
    pub(crate) fn stop_words(self) -> &'static [&'static str] {
        match self {
            Self::English => english::WORDS,
            Self::German => german::WORDS,
            _ => &[],
        }
    }
}
