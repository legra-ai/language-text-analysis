//! The normalized [`Term`] token type.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A single normalized search term — the output unit of the analysis
/// pipeline and the key under which the full-text index stores
/// postings.
///
/// A `Term` is always the product of [`Analyzer::analyze`]: normalized
/// (lowercased, accent-stripped), tokenized, stop-word-filtered, and
/// optionally stemmed. The same pipeline runs at index time and query
/// time, so a query `Term` is byte-for-byte comparable with the indexed
/// `Term` it should match.
///
/// Construct one only from already-analyzed text. [`from_normalized`]
/// exists for the pipeline itself and for callers that have run the
/// identical analysis; it does **not** normalize on your behalf.
///
/// [`Analyzer::analyze`]: crate::Analyzer::analyze
/// [`from_normalized`]: Term::from_normalized
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Term(String);

impl Term {
    /// Wrap an already-normalized token.
    ///
    /// The pipeline guarantees the input has been normalized; this
    /// constructor performs no normalization itself.
    #[must_use]
    pub fn from_normalized(token: impl Into<String>) -> Self {
        Self(token.into())
    }

    /// The token text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the term, yielding its inner string.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
