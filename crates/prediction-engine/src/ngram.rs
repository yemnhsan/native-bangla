//! Simple bigram / trigram models for next-word prediction.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NGramModel {
    /// bigram: "prev" -> (next -> count)
    pub bigrams: HashMap<String, HashMap<String, u32>>,
    /// trigram: "w1 w2" -> (next -> count)
    pub trigrams: HashMap<String, HashMap<String, u32>>,
}

impl NGramModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn observe(&mut self, words: &[&str]) {
        for w in words.windows(2) {
            *self
                .bigrams
                .entry(w[0].to_string())
                .or_default()
                .entry(w[1].to_string())
                .or_default() += 1;
        }
        for w in words.windows(3) {
            let key = format!("{} {}", w[0], w[1]);
            *self
                .trigrams
                .entry(key)
                .or_default()
                .entry(w[2].to_string())
                .or_default() += 1;
        }
    }

    pub fn load_seed_pairs(&mut self, pairs: &[(&str, &str, u32)]) {
        for (a, b, count) in pairs {
            *self
                .bigrams
                .entry((*a).to_string())
                .or_default()
                .entry((*b).to_string())
                .or_default() += *count;
        }
    }

    pub fn load_seed_triples(&mut self, triples: &[(&str, &str, &str, u32)]) {
        for (a, b, c, count) in triples {
            let key = format!("{a} {b}");
            *self
                .trigrams
                .entry(key)
                .or_default()
                .entry((*c).to_string())
                .or_default() += *count;
            // Also reinforce bigram
            *self
                .bigrams
                .entry((*b).to_string())
                .or_default()
                .entry((*c).to_string())
                .or_default() += *count;
        }
    }

    /// Predict next words given preceding context (most recent last).
    pub fn predict_next(&self, context: &[&str], limit: usize) -> Vec<(String, f64)> {
        if context.is_empty() {
            return vec![];
        }

        let mut scores: HashMap<String, f64> = HashMap::new();

        if context.len() >= 2 {
            let key = format!("{} {}", context[context.len() - 2], context[context.len() - 1]);
            if let Some(map) = self.trigrams.get(&key) {
                let total: u32 = map.values().sum::<u32>().max(1);
                for (w, c) in map {
                    *scores.entry(w.clone()).or_default() += 2.0 * (*c as f64) / (total as f64);
                }
            }
        }

        let prev = context[context.len() - 1];
        if let Some(map) = self.bigrams.get(prev) {
            let total: u32 = map.values().sum::<u32>().max(1);
            for (w, c) in map {
                *scores.entry(w.clone()).or_default() += (*c as f64) / (total as f64);
            }
        }

        let mut ranked: Vec<_> = scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked.truncate(limit);
        ranked
    }
}

/// Seed common Bengali conversational patterns (illustrative frequency).
pub fn seed_bengali_model() -> NGramModel {
    let mut m = NGramModel::new();
    m.load_seed_pairs(&[
        ("আমি", "তোমাকে", 80),
        ("আমি", "আজ", 120),
        ("আমি", "একটা", 90),
        ("আমি", "বাংলায়", 70),
        ("তুমি", "কেমন", 100),
        ("কেমন", "আছো", 150),
        ("কেমন", "আছেন", 110),
        ("আজ", "স্কুলে", 60),
        ("আজ", "বাড়িতে", 70),
        ("আজ", "অনেক", 85),
        ("আমার", "নাম", 95),
        ("ধন্যবাদ", "আপনাকে", 50),
    ]);
    m.load_seed_triples(&[
        ("আমি", "আজ", "স্কুলে", 40),
        ("আমি", "আজ", "বাড়িতে", 35),
        ("আমি", "আজ", "অনেক", 45),
        ("তুমি", "কেমন", "আছো", 100),
        ("আমি", "বাংলায়", "লিখতে", 55),
        ("বাংলায়", "লিখতে", "চাই", 60),
    ]);
    m
}
