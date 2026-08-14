//! Default German stop-word list (already normalized: lowercase and
//! accent-stripped, e.g. `für` → `fur`, `über` → `uber`, `wäre` →
//! `ware` — matching the post-normalization form of a German token).

/// Common German stop words removed by default, stored accent-stripped
/// so they compare directly against normalized tokens.
pub(crate) const WORDS: &[&str] = &[
    "aber", "alle", "als", "also", "am", "an", "auch", "auf", "aus", "bei", "bin", "bis", "bist",
    "da", "damit", "dann", "das", "dass", "dem", "den", "der", "des", "die", "dies", "doch",
    "dort", "du", "durch", "ein", "eine", "einem", "einen", "einer", "eines", "er", "es", "fur",
    "gegen", "habe", "haben", "hat", "hatte", "hier", "ich", "ihr", "im", "in", "ist", "ja",
    "kann", "kein", "man", "mehr", "mein", "mit", "muss", "nach", "nicht", "noch", "nun", "nur",
    "ob", "oder", "ohne", "schon", "sehr", "sein", "seine", "sich", "sie", "sind", "so", "uber",
    "um", "und", "uns", "unter", "vom", "von", "vor", "war", "ware", "warum", "was", "weil",
    "wenn", "werden", "wie", "wir", "wird", "wo", "zu", "zum", "zur",
];
