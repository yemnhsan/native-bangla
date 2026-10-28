//! Versioned dictionary load/save for Native Bangla.

use crate::trie::{DictEntry, Trie};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Error)]
pub enum DictionaryError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("invalid dictionary: {0}")]
    Invalid(String),
}

/// On-disk dictionary document format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryDocument {
    pub version: u32,
    pub name: String,
    pub locale: String,
    pub license: String,
    pub source: String,
    pub entries: Vec<DictEntry>,
}

impl DictionaryDocument {
    pub fn validate(&self) -> Result<(), DictionaryError> {
        if self.version == 0 {
            return Err(DictionaryError::Invalid("version must be >= 1".into()));
        }
        if self.entries.is_empty() {
            return Err(DictionaryError::Invalid("dictionary has no entries".into()));
        }
        Ok(())
    }

    pub fn load_path(path: impl AsRef<Path>) -> Result<Self, DictionaryError> {
        let data = fs::read_to_string(path)?;
        let doc: Self = serde_json::from_str(&data)?;
        doc.validate()?;
        Ok(doc)
    }

    pub fn save_path(&self, path: impl AsRef<Path>) -> Result<(), DictionaryError> {
        self.validate()?;
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    /// Normalize Unicode, dedupe by word keeping max frequency.
    pub fn normalize(mut self) -> Self {
        use std::collections::HashMap;
        let mut map: HashMap<String, DictEntry> = HashMap::new();
        for mut e in self.entries.drain(..) {
            e.word = e.word.nfc().collect::<String>().trim().to_string();
            if e.word.is_empty() {
                continue;
            }
            map.entry(e.word.clone())
                .and_modify(|existing| {
                    if e.frequency > existing.frequency {
                        existing.frequency = e.frequency;
                    }
                    if existing.phonetic.is_none() {
                        existing.phonetic = e.phonetic.clone();
                    }
                })
                .or_insert(e);
        }
        let mut entries: Vec<_> = map.into_values().collect();
        entries.sort_by(|a, b| b.frequency.cmp(&a.frequency).then(a.word.cmp(&b.word)));
        self.entries = entries;
        self
    }

    pub fn build_trie(&self) -> Trie {
        let mut trie = Trie::new();
        for e in &self.entries {
            trie.insert(&e.word, e.frequency);
        }
        trie
    }

    /// Secondary index: phonetic latin → Bengali entries.
    pub fn build_phonetic_trie(&self) -> Trie {
        let mut trie = Trie::new();
        for e in &self.entries {
            if let Some(ref ph) = e.phonetic {
                trie.insert(ph, e.frequency);
            }
        }
        trie
    }
}

/// Runtime dictionary handle.
#[derive(Debug, Clone)]
pub struct Dictionary {
    pub meta_name: String,
    pub version: u32,
    pub bangla_trie: Trie,
    pub phonetic_trie: Trie,
    pub entries: Vec<DictEntry>,
}

impl Dictionary {
    pub fn from_document(doc: DictionaryDocument) -> Self {
        let doc = doc.normalize();
        let bangla_trie = doc.build_trie();
        let phonetic_trie = doc.build_phonetic_trie();
        Self {
            meta_name: doc.name,
            version: doc.version,
            bangla_trie,
            phonetic_trie,
            entries: doc.entries,
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, DictionaryError> {
        Ok(Self::from_document(DictionaryDocument::load_path(path)?))
    }

    pub fn word_count(&self) -> usize {
        self.entries.len()
    }

    pub fn prefix_bangla(&self, prefix: &str, limit: usize) -> Vec<DictEntry> {
        self.bangla_trie.prefix_search(prefix, limit)
    }

    pub fn prefix_phonetic(&self, prefix: &str, limit: usize) -> Vec<DictEntry> {
        // Map phonetic prefixes to bangla words via entry phonetic fields
        let mut matched: Vec<DictEntry> = self
            .entries
            .iter()
            .filter(|e| {
                e.phonetic
                    .as_ref()
                    .map(|p| p.starts_with(prefix))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        matched.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        matched.truncate(limit);
        if matched.is_empty() {
            // Fall back to phonetic trie leaf words (latin keys) — caller maps via overrides
            return self.phonetic_trie.prefix_search(prefix, limit);
        }
        matched
    }
}
