//! Definition-of-done tests for the analysis pipeline.

use crate::{Analyzer, Language, StopWordPolicy, Term};

fn strings(terms: Vec<Term>) -> Vec<String> {
    terms.into_iter().map(Term::into_inner).collect()
}

/// Definition-of-done 1: accent stripping — `Zürich` normalizes to `zurich`.
#[test]
fn strips_accents_and_lowercases() {
    let analyzer = Analyzer::new();
    assert_eq!(strings(analyzer.analyze("Zürich", None)), vec!["zurich"]);
}

/// Definition-of-done 2: a CJK string tokenizes to per-character terms.
#[test]
fn cjk_tokenizes_per_character() {
    let analyzer = Analyzer::new();
    assert_eq!(
        strings(analyzer.analyze("日本語", None)),
        vec!["日", "本", "語"],
    );
}

/// Definition-of-done 3: stop words removed under the default list, retained
/// under [`StopWordPolicy::None`].
#[test]
fn stop_words_respect_policy() {
    let default = Analyzer::new();
    assert_eq!(
        strings(default.analyze("the knowledge graph", None)),
        vec!["knowledge", "graph"],
        "default policy drops the English stop word 'the'",
    );

    let keep_all = Analyzer::builder().stop_words(StopWordPolicy::None).build();
    assert_eq!(
        strings(keep_all.analyze("the knowledge graph", None)),
        vec!["the", "knowledge", "graph"],
        "StopWordPolicy::None retains every token",
    );
}

/// Definition-of-done 4: English stemming maps `running` → `run` when enabled,
/// and leaves it untouched when off.
#[test]
fn english_stemming_is_opt_in() {
    let off = Analyzer::new();
    assert_eq!(strings(off.analyze("running", None)), vec!["running"]);

    let on = Analyzer::builder().stemming(true).build();
    assert_eq!(strings(on.analyze("running", None)), vec!["run"]);
}

/// Definition-of-done 5: an `@de` tag selects the German stop-word list and the
/// German stemmer.
#[test]
fn german_tag_selects_german_analysis() {
    assert_eq!(Language::from_tag("de"), Some(Language::German));
    assert_eq!(Language::from_tag("de-DE"), Some(Language::German));

    // German stop words ("und", "der") are removed only when the German
    // list is selected — the English default list contains neither.
    let analyzer = Analyzer::new();
    assert_eq!(
        strings(analyzer.analyze("und der Test", Some("de"))),
        vec!["test"],
        "German list drops 'und'/'der'",
    );
    assert_eq!(
        strings(analyzer.analyze("und der Test", Some("en"))),
        vec!["und", "der", "test"],
        "English list keeps the German function words",
    );

    // The German stemmer reduces a word the English stemmer leaves
    // alone — proving the algorithm is tag-selected.
    let stemming = Analyzer::builder().stemming(true).build();
    let de = strings(stemming.analyze("Stunden", Some("de")));
    let en = strings(stemming.analyze("Stunden", Some("en")));
    assert_ne!(de, en, "@de must select the German stemmer");
    assert_ne!(de, vec!["stunden"], "German stemmer reduces the word");
}

/// Definition-of-done 6: the same input yields identical terms across call
/// sites (the index-time and query-time invariant).
#[test]
fn analysis_is_deterministic_across_call_sites() {
    let index_side = Analyzer::builder().stemming(true).build();
    let query_side = Analyzer::builder().stemming(true).build();
    let indexed = index_side.analyze("Running Knowledge Graphs", None);
    let queried = query_side.analyze("Running Knowledge Graphs", None);
    assert_eq!(indexed, queried);
}

/// Term frequency is preserved: repeated tokens are not deduplicated.
#[test]
fn repeated_tokens_are_preserved() {
    let analyzer = Analyzer::new();
    assert_eq!(
        strings(analyzer.analyze("graph graph graph", None)),
        vec!["graph", "graph", "graph"],
    );
}
