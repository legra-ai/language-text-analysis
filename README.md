# language-text-analysis

[![Crates.io](https://img.shields.io/crates/v/language-text-analysis.svg)](https://crates.io/crates/language-text-analysis)
[![Documentation](https://docs.rs/language-text-analysis/badge.svg)](https://docs.rs/language-text-analysis)
[![CI](https://github.com/legra-ai/language-text-analysis/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/language-text-analysis/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Downloads](https://img.shields.io/crates/d/language-text-analysis.svg)](https://crates.io/crates/language-text-analysis)

A small, deterministic multilingual text-analysis pipeline for search indexes
and document-processing applications.

The pipeline is:

```text
normalize → tokenize → stop-word removal → optional stemming
```

It provides one analysis path for both indexing and querying, which keeps the
terms stored in an index comparable with the terms produced for a search
query.

## Behavior

- Unicode lowercasing and accent stripping (`Zürich` becomes `zurich`).
- Unicode word segmentation, including character-level segmentation for CJK
  ideographs.
- Optional default stop-word removal for English and German.
- Optional Snowball stemming for English, German, French, Spanish, Italian,
  Dutch, Portuguese, Swedish, Norwegian, Danish, Finnish, and Russian.
- BCP-47 language-tag selection with a configurable fallback language.
- Repeated terms remain repeated, preserving term frequency for callers that
  score results.

```rust
use language_text_analysis::{
    Analyzer,
    Language,
};

let analyzer = Analyzer::builder()
    .default_language(Language::English)
    .stemming(true)
    .build();

let terms = analyzer.analyze("Zürich is running", Some("en-US"));
assert_eq!(
    terms.into_iter().map(|term| term.into_inner()).collect::<Vec<_>>(),
    ["zurich", "run"],
);
```

`Analyzer::new()` provides English defaults, removes the bundled English
stop-word list, and leaves stemming disabled. Applications that need maximum
recall can select `StopWordPolicy::None`.

The analyzer returns a `Vec<Term>` for one input string. It does not own an
index, ranking algorithm, corpus, or application-specific query language.

## License

Copyright © 2026 `DataRoad Inc`, Delaware, USA, trading as Legra.

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
