#![doc = include_str!("../README.md")]

mod analyzer;
mod language;
mod normalize;
mod stopwords;
mod term;
mod tokenize;

pub use analyzer::{
    Analyzer,
    AnalyzerBuilder,
    StopWordPolicy,
};
pub use language::Language;
pub use term::Term;

#[cfg(test)]
mod tests;
