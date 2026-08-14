//! The [`Analyzer`]: the single entry point that runs the full
//! analysis pipeline at both index time and query time.

use rust_stemmers::Stemmer;

use crate::language::Language;
use crate::normalize::normalize;
use crate::term::Term;
use crate::tokenize::tokenize;

/// How the analyzer treats stop words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopWordPolicy {
    /// Remove the default stop words for the analyzed language.
    DefaultForLanguage,
    /// Keep every token (maximum recall).
    None,
}

impl StopWordPolicy {
    /// Whether `token` should be dropped for `language` under this
    /// policy.
    fn removes(self, language: Language, token: &str) -> bool {
        match self {
            Self::DefaultForLanguage => language.stop_words().contains(&token),
            Self::None => false,
        }
    }
}

/// The text-analysis pipeline: normalize → tokenize → stop-word removal
/// → optional stemming.
///
/// One `Analyzer` is configured per branch and used for **both**
/// indexing and querying, so an indexed term and the query term that
/// should match it pass through identical transformations. Build one
/// with [`Analyzer::builder`]; [`Analyzer::new`] gives the English
/// default (default stop words, no stemming).
#[derive(Clone, Debug)]
pub struct Analyzer {
    default_language: Language,
    stop_words: StopWordPolicy,
    stemming: bool,
}

impl Analyzer {
    /// The English-default analyzer: default English stop words,
    /// stemming off.
    #[must_use]
    pub fn new() -> Self {
        Self {
            default_language: Language::English,
            stop_words: StopWordPolicy::DefaultForLanguage,
            stemming: false,
        }
    }

    /// Start building a customized analyzer.
    #[must_use]
    pub fn builder() -> AnalyzerBuilder {
        AnalyzerBuilder::new()
    }

    /// The language applied to text whose tag is absent or
    /// unrecognized.
    #[must_use]
    pub fn default_language(&self) -> Language {
        self.default_language
    }

    /// Run the pipeline over `text`, producing its ordered terms.
    ///
    /// `lang_tag` is the text's optional BCP-47 language tag. When present
    /// and recognized it selects the stemmer and stop-word list; otherwise
    /// the analyzer's [`default_language`] applies.
    ///
    /// Duplicate terms are preserved (term frequency matters for
    /// scoring), in their original order.
    ///
    /// [`default_language`]: Analyzer::default_language
    #[must_use]
    pub fn analyze(&self, text: &str, lang_tag: Option<&str>) -> Vec<Term> {
        let language = lang_tag
            .and_then(Language::from_tag)
            .unwrap_or(self.default_language);
        let normalized = normalize(text);
        let stemmer = self
            .stemming
            .then(|| Stemmer::create(language.stemmer_algorithm()));

        // bounded: holds the tokens of a single literal/document, not a
        // workspace-scaled relation; callers feed one literal at a time.
        let mut terms = Vec::new();
        for token in tokenize(&normalized) {
            if self.stop_words.removes(language, token) {
                continue;
            }
            let term = match &stemmer {
                Some(stemmer) => Term::from_normalized(stemmer.stem(token).into_owned()),
                None => Term::from_normalized(token),
            };
            terms.push(term);
        }
        terms
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for an [`Analyzer`] with a non-default configuration.
#[derive(Clone, Debug)]
pub struct AnalyzerBuilder {
    default_language: Language,
    stop_words: StopWordPolicy,
    stemming: bool,
}

impl AnalyzerBuilder {
    fn new() -> Self {
        Self {
            default_language: Language::English,
            stop_words: StopWordPolicy::DefaultForLanguage,
            stemming: false,
        }
    }

    /// Set the language used for untagged or unrecognized-tag text.
    #[must_use]
    pub fn default_language(mut self, language: Language) -> Self {
        self.default_language = language;
        self
    }

    /// Set the stop-word policy.
    #[must_use]
    pub fn stop_words(mut self, policy: StopWordPolicy) -> Self {
        self.stop_words = policy;
        self
    }

    /// Enable or disable Snowball stemming.
    #[must_use]
    pub fn stemming(mut self, enabled: bool) -> Self {
        self.stemming = enabled;
        self
    }

    /// Finish building the analyzer.
    #[must_use]
    pub fn build(self) -> Analyzer {
        Analyzer {
            default_language: self.default_language,
            stop_words: self.stop_words,
            stemming: self.stemming,
        }
    }
}

impl Default for AnalyzerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
