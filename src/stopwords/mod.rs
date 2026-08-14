//! Built-in default stop-word lists.
//!
//! Each list is stored **already normalized** — lowercase and
//! accent-stripped, matching the form a token has after the
//! [`normalize`](crate) stage — so stop-word removal can compare
//! tokens directly without re-normalizing the list. The lists are
//! deliberately compact common-word sets, not exhaustive linguistic
//! corpora; callers wanting maximum recall select
//! [`StopWordPolicy::None`](crate::StopWordPolicy).

pub(crate) mod english;
pub(crate) mod german;
