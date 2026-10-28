//! Privacy-preserving local personalization for Native Predict.
//!
//! Stores aggregated selection counts only — never raw keystroke streams
//! or full typed sentences.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersonalError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalStore {
    pub version: u32,
    /// phonetic_key -> (bengali_word -> count)
    pub selections: HashMap<String, HashMap<String, u32>>,
    /// Custom vocabulary approved by the user
    pub custom_words: Vec<String>,
    pub learning_enabled: bool,
}

impl PersonalStore {
    pub fn new() -> Self {
        Self {
            version: 1,
            selections: HashMap::new(),
            custom_words: Vec::new(),
            learning_enabled: true,
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, PersonalError> {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), PersonalError> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn record_selection(&mut self, phonetic: &str, bangla: &str) {
        if !self.learning_enabled {
            return;
        }
        *self
            .selections
            .entry(phonetic.to_ascii_lowercase())
            .or_default()
            .entry(bangla.to_string())
            .or_default() += 1;
    }

    pub fn counts_for_phonetic(&self, phonetic: &str) -> HashMap<String, u32> {
        self.selections
            .get(&phonetic.to_ascii_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    pub fn word_boosts(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();
        for inner in self.selections.values() {
            for (word, count) in inner {
                *map.entry(word.clone()).or_default() += *count;
            }
        }
        for w in &self.custom_words {
            *map.entry(w.clone()).or_default() += 10;
        }
        map
    }

    pub fn clear(&mut self) {
        self.selections.clear();
        self.custom_words.clear();
    }

    pub fn add_custom_word(&mut self, word: String) {
        if !self.custom_words.contains(&word) {
            self.custom_words.push(word);
        }
    }
}
