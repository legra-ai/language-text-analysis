//! Text normalization: Unicode lowercase, accent stripping, and
//! whitespace collapse — the first stage of the analysis pipeline.

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

/// Normalize text for analysis.
///
/// 1. Unicode lowercase (`str::to_lowercase`).
/// 2. Strip combining diacritical marks: NFD-decompose, drop every `\p{M}` code
///    point, NFC-recompose — so `Zürich` becomes `zurich`.
/// 3. Collapse every run of whitespace to a single ASCII space and trim the
///    ends.
///
/// Tokenization runs on the result, so this stage only has to leave the
/// text in a canonical comparable form.
pub(crate) fn normalize(text: &str) -> String {
    let lowered = text.to_lowercase();
    let stripped: String = lowered
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .nfc()
        .collect();
    collapse_whitespace(&stripped)
}

/// Collapse internal whitespace runs to single spaces and trim ends.
fn collapse_whitespace(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            pending_space = !out.is_empty();
        } else {
            if pending_space {
                out.push(' ');
                pending_space = false;
            }
            out.push(ch);
        }
    }
    out
}
