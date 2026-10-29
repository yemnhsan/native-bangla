//! Native Predict — layered suggestion engine.

use crate::ngram::{seed_bengali_model, NGramModel};
use crate::personal::PersonalStore;
use crate::ranker::{rank_candidates, RankWeights, ScoredCandidate};
use dictionary_core::{DictEntry, Dictionary};
use phonetic_engine::{ambiguous_candidates, transliterate, PhoneticConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Prediction request from the IME.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictRequest {
    /// Current phonetic / latin buffer for the word being typed.
    pub phonetic_prefix: String,
    /// Preceding committed words (Bengali), most recent last.
    #[serde(default)]
    pub context_words: Vec<String>,
    /// Max suggestions to return (typically 3–5).
    pub limit: usize,
    /// Whether next-word mode (empty phonetic, use context only).
    #[serde(default)]
    pub next_word: bool,
}

/// A single suggestion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Suggestion {
    pub text: String,
    pub score: f64,
    pub source: SuggestionSource,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionSource {
    Phonetic,
    Dictionary,
    Context,
    Personal,
    Autocorrect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictResponse {
    pub suggestions: Vec<Suggestion>,
    /// Primary transliteration (deterministic).
    pub primary: String,
}

#[derive(Debug, Clone)]
pub struct PredictorConfig {
    pub phonetic: PhoneticConfig,
    pub weights: RankWeights,
    pub min_confidence: f64,
    pub enable_next_word: bool,
    pub enable_personal: bool,
}

impl Default for PredictorConfig {
    fn default() -> Self {
        Self {
            phonetic: PhoneticConfig::default(),
            weights: RankWeights::default(),
            min_confidence: 0.05,
            enable_next_word: true,
            enable_personal: true,
        }
    }
}

/// Layered prediction engine.
pub struct Predictor {
    pub dictionary: Dictionary,
    pub ngrams: NGramModel,
    pub personal: PersonalStore,
    pub config: PredictorConfig,
}

impl Predictor {
    pub fn new(dictionary: Dictionary) -> Self {
        Self {
            dictionary,
            ngrams: seed_bengali_model(),
            personal: PersonalStore::new(),
            config: PredictorConfig::default(),
        }
    }

    pub fn predict(&self, req: &PredictRequest) -> PredictResponse {
        let limit = req.limit.clamp(1, 10);

        if req.next_word || req.phonetic_prefix.is_empty() {
            return self.predict_next_word(&req.context_words, limit);
        }

        let primary = transliterate(&req.phonetic_prefix, &self.config.phonetic);
        let mut suggestions = Vec::new();

        // Layer 1: deterministic phonetic + ambiguous variants
        let variants = ambiguous_candidates(&req.phonetic_prefix, &self.config.phonetic);
        for (i, v) in variants.iter().enumerate() {
            suggestions.push(Suggestion {
                text: v.clone(),
                score: 10.0 - (i as f64) * 0.1,
                source: SuggestionSource::Phonetic,
            });
        }

        // Layer 2+3: dictionary prefix on phonetic keys
        let dict_hits = self
            .dictionary
            .prefix_phonetic(&req.phonetic_prefix.to_ascii_lowercase(), limit * 3);

        // Also search bangla prefix of primary transliteration
        let mut bangla_hits = self.dictionary.prefix_bangla(&primary, limit * 2);
        for h in dict_hits {
            if !bangla_hits.iter().any(|e| e.word == h.word) {
                bangla_hits.push(h);
            }
        }

        // If dictionary phonetic entries store latin as word, map via transliterate
        let mut entries: Vec<DictEntry> = bangla_hits;
        // Ensure primary is represented
        if !entries.iter().any(|e| e.word == primary) && !primary.is_empty() {
            entries.insert(
                0,
                DictEntry {
                    word: primary.clone(),
                    frequency: 1,
                    phonetic: Some(req.phonetic_prefix.clone()),
                },
            );
        }

        let personal_boosts = if self.config.enable_personal {
            self.personal.word_boosts()
        } else {
            HashMap::new()
        };

        // Context boosts from n-gram next-word overlap
        let mut context_boosts = HashMap::new();
        if !req.context_words.is_empty() {
            let ctx: Vec<&str> = req.context_words.iter().map(|s| s.as_str()).collect();
            for (w, score) in self.ngrams.predict_next(&ctx, 20) {
                context_boosts.insert(w, score);
            }
        }

        let ranked = rank_candidates(
            &entries,
            &personal_boosts,
            &context_boosts,
            Some(&primary),
            &self.config.weights,
            limit * 2,
            self.config.min_confidence,
        );

        for ScoredCandidate { word, score, .. } in ranked {
            if suggestions.iter().any(|s| s.text == word) {
                // Boost existing
                if let Some(s) = suggestions.iter_mut().find(|s| s.text == word) {
                    s.score += score;
                    if personal_boosts.contains_key(&word) {
                        s.source = SuggestionSource::Personal;
                    }
                }
                continue;
            }
            let source = if personal_boosts.contains_key(&word) {
                SuggestionSource::Personal
            } else if context_boosts.contains_key(&word) {
                SuggestionSource::Context
            } else {
                SuggestionSource::Dictionary
            };
            suggestions.push(Suggestion {
                text: word,
                score,
                source,
            });
        }

        suggestions.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions.truncate(limit);

        // Suppress low-confidence noisy sets
        if suggestions.len() > 1 {
            let best = suggestions[0].score;
            suggestions.retain(|s| s.score >= best * 0.10 || s.text == primary);
            suggestions.truncate(limit);
        }

        PredictResponse {
            suggestions,
            primary,
        }
    }

    fn predict_next_word(&self, context: &[String], limit: usize) -> PredictResponse {
        if !self.config.enable_next_word || context.is_empty() {
            return PredictResponse {
                suggestions: vec![],
                primary: String::new(),
            };
        }
        let ctx: Vec<&str> = context.iter().map(|s| s.as_str()).collect();
        let ranked = self.ngrams.predict_next(&ctx, limit);
        let suggestions: Vec<Suggestion> = ranked
            .into_iter()
            .filter(|(_, score)| *score >= self.config.min_confidence)
            .map(|(text, score)| Suggestion {
                text,
                score,
                source: SuggestionSource::Context,
            })
            .collect();

        PredictResponse {
            primary: suggestions
                .first()
                .map(|s| s.text.clone())
                .unwrap_or_default(),
            suggestions,
        }
    }

    pub fn record_acceptance(&mut self, phonetic: &str, bangla: &str) {
        self.personal.record_selection(phonetic, bangla);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dictionary_core::DictionaryDocument;

    fn sample_dict() -> Dictionary {
        let doc = DictionaryDocument {
            version: 1,
            name: "test".into(),
            locale: "bn".into(),
            license: "MIT".into(),
            source: "test".into(),
            entries: vec![
                DictEntry {
                    word: "আমি".into(),
                    frequency: 1000,
                    phonetic: Some("ami".into()),
                },
                DictEntry {
                    word: "আমার".into(),
                    frequency: 900,
                    phonetic: Some("amar".into()),
                },
                DictEntry {
                    word: "আমাদের".into(),
                    frequency: 800,
                    phonetic: Some("amader".into()),
                },
                DictEntry {
                    word: "আপনি".into(),
                    frequency: 700,
                    phonetic: Some("apni".into()),
                },
                DictEntry {
                    word: "বাংলা".into(),
                    frequency: 950,
                    phonetic: Some("bangla".into()),
                },
                DictEntry {
                    word: "বাংলাদেশ".into(),
                    frequency: 850,
                    phonetic: Some("bangladesh".into()),
                },
                DictEntry {
                    word: "বাংলাদেশি".into(),
                    frequency: 400,
                    phonetic: Some("bangladeshi".into()),
                },
            ],
        };
        Dictionary::from_document(doc)
    }

    #[test]
    fn prefix_am() {
        let p = Predictor::new(sample_dict());
        let resp = p.predict(&PredictRequest {
            phonetic_prefix: "am".into(),
            context_words: vec![],
            limit: 5,
            next_word: false,
        });
        assert!(!resp.suggestions.is_empty());
        let texts: Vec<_> = resp.suggestions.iter().map(|s| s.text.as_str()).collect();
        assert!(
            texts.iter().any(|t| t.starts_with('আ')),
            "expected bangla suggestions, got {texts:?}"
        );
    }

    #[test]
    fn prefix_bang() {
        let p = Predictor::new(sample_dict());
        let resp = p.predict(&PredictRequest {
            phonetic_prefix: "bang".into(),
            context_words: vec![],
            limit: 5,
            next_word: false,
        });
        let texts: Vec<_> = resp.suggestions.iter().map(|s| s.text.clone()).collect();
        assert!(texts.iter().any(|t| t.contains("বাংলা")));
    }

    #[test]
    fn next_word_after_ami() {
        let p = Predictor::new(sample_dict());
        let resp = p.predict(&PredictRequest {
            phonetic_prefix: String::new(),
            context_words: vec!["আমি".into()],
            limit: 5,
            next_word: true,
        });
        assert!(!resp.suggestions.is_empty());
    }

    #[test]
    fn next_word_kemon() {
        let p = Predictor::new(sample_dict());
        let resp = p.predict(&PredictRequest {
            phonetic_prefix: String::new(),
            context_words: vec!["তুমি".into(), "কেমন".into()],
            limit: 5,
            next_word: true,
        });
        assert!(resp.suggestions.iter().any(|s| s.text == "আছো"));
    }
}
