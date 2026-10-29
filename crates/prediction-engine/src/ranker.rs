//! Candidate ranking for Native Predict.

use dictionary_core::DictEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ranking weights (tunable).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankWeights {
    pub frequency: f64,
    pub personal: f64,
    pub context: f64,
    pub exact_phonetic: f64,
}

impl Default for RankWeights {
    fn default() -> Self {
        Self {
            frequency: 5.0,
            personal: 2.5,
            context: 1.8,
            exact_phonetic: 3.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScoredCandidate {
    pub word: String,
    pub score: f64,
    pub frequency: u32,
}

/// Rank dictionary entries with optional personal and context boosts.
pub fn rank_candidates(
    entries: &[DictEntry],
    personal: &HashMap<String, u32>,
    context_boosts: &HashMap<String, f64>,
    exact_match: Option<&str>,
    weights: &RankWeights,
    limit: usize,
    min_confidence: f64,
) -> Vec<ScoredCandidate> {
    let max_freq = entries.iter().map(|e| e.frequency).max().unwrap_or(1).max(1) as f64;

    let mut scored: Vec<ScoredCandidate> = entries
        .iter()
        .map(|e| {
            let freq_norm = (e.frequency as f64) / max_freq;
            let pers = personal.get(&e.word).copied().unwrap_or(0) as f64;
            let pers_norm = (pers / (pers + 5.0)).min(1.0);
            let ctx = context_boosts.get(&e.word).copied().unwrap_or(0.0);
            let exact = if exact_match == Some(e.word.as_str()) {
                1.0
            } else {
                0.0
            };

            let score = weights.frequency * freq_norm
                + weights.personal * pers_norm
                + weights.context * ctx
                + weights.exact_phonetic * exact;

            ScoredCandidate {
                word: e.word.clone(),
                score,
                frequency: e.frequency,
            }
        })
        .filter(|c| c.score >= min_confidence)
        .collect();

    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(b.frequency.cmp(&a.frequency))
            .then(a.word.cmp(&b.word))
    });
    scored.truncate(limit);
    scored
}
